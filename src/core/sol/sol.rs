use bigdecimal::ToPrimitive;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig, message::Message, program_pack::Pack, pubkey::Pubkey,
    signature::Signature, transaction::Transaction,
};
use solana_transaction_status::UiTransactionEncoding;
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    instruction::{transfer_checked, TokenInstruction},
    state::Mint,
};
use sqlx::{PgPool, Row};
use std::{env, str::FromStr};

use crate::{
    auth::jwt::generate_payment_jwt,
    db::migrations::payments::{
        inserts_and_updates::{ADD_PAYMENT, ADD_PENDING_PAYMENT, DELETE_PENDING_PAYMENT},
        select_queries::GET_PENDING_PAYMENT,
    },
    error::StabuseError,
    merchant::merchant::get_merchant_network_address,
    network::network::get_network_and_asset_address_with_chain_id,
    types::types::{PaymentAuthDetails, PendingPayment},
    utils::utils::{generate_webhook_url, get_solana_network_identifier, get_token_decimals},
};

const REQUIRED_CONFIRMATIONS: u64 = 12;

pub async fn create_payment_transaction(
    pool: &PgPool,
    rpc_url: &str,
    payer: &str,
    merchant_id: i32,
    asset: &str,
    amount: u64,
) -> Result<(Transaction, PaymentAuthDetails), Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new(rpc_url.to_string());
    let chain_id = get_solana_network_identifier(rpc_url)?;
    let merchant = get_merchant_network_address(pool, merchant_id, chain_id).await?;
    let (network, token_mint) =
        get_network_and_asset_address_with_chain_id(pool, asset, chain_id as u64).await?;
    let payer_pubkey = Pubkey::from_str(payer)?;
    let merchant_pubkey = Pubkey::from_str(merchant.as_str())?;
    let token_mint_pubkey = Pubkey::from_str(token_mint.as_str())?;
    let account_data = rpc_client.get_account_data(&token_mint_pubkey)?;
    let mint = Mint::unpack(&account_data)?;
    let decimals = mint.decimals;

    let user_token_account = get_associated_token_address(&payer_pubkey, &token_mint_pubkey);
    let merchant_token_account = get_associated_token_address(&merchant_pubkey, &token_mint_pubkey);

    let transfer_instruction = transfer_checked(
        &spl_token::id(),
        &user_token_account,
        &token_mint_pubkey,
        &merchant_token_account,
        &payer_pubkey,
        &[],
        amount,
        decimals,
    )?;

    let message = Message::new(&[transfer_instruction], Some(&payer_pubkey));
    let _recent_blockhash = rpc_client.get_latest_blockhash()?;

    let transaction = Transaction::new_unsigned(message);

    let (webhook_url, timestamp) = generate_webhook_url(merchant_id, payer, amount);
    tracing::info!("Generated Webhook URL: {}", webhook_url);
    tracing::info!("Generated Timestamp: {}", timestamp);

    let pending_payment_id: i32 = sqlx::query(ADD_PENDING_PAYMENT)
        .bind(merchant_id)
        .bind(payer)
        .bind(amount.to_string())
        .bind(asset)
        .bind(network.clone())
        .bind(webhook_url.clone())
        .fetch_one(pool)
        .await?
        .get(0);

    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let token = generate_payment_jwt(
        pending_payment_id,
        &jwt_secret,
        rpc_url.to_string(),
        network,
    )?;

    let auth_details = PaymentAuthDetails {
        jwt_token: token,
        webhook_url: webhook_url.clone(),
    };

    Ok((transaction, auth_details))
}

pub async fn verify_sol_signed_transaction(
    pool: &PgPool,
    pending_payment_id: i32,
    rpc_url: &str,
    tx_hash: &str,
) -> Result<(i32, String), StabuseError> {
    let pending_payment = sqlx::query_as::<_, PendingPayment>(GET_PENDING_PAYMENT)
        .bind(pending_payment_id)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    let rpc_client = RpcClient::new(rpc_url.to_string());
    let chain_id = get_solana_network_identifier(rpc_url)?;

    let (network, _token_mint) =
        get_network_and_asset_address_with_chain_id(pool, &pending_payment.asset, chain_id as u64)
            .await?;

    let signature = Signature::from_str(tx_hash).map_err(|e| {
        StabuseError::Internal(format!("Failed to parse transaction signature: {}", e))
    })?;

    let transaction = rpc_client
        .get_transaction_with_config(
            &signature,
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Base64),
                commitment: Some(CommitmentConfig::confirmed()),
                ..Default::default()
            },
        )
        .map_err(|e| StabuseError::Internal(format!("Failed to fetch transaction: {}", e)))?;

    let current_slot = rpc_client
        .get_slot()
        .map_err(|e| StabuseError::Internal(format!("Failed to get current slot: {}", e)))?;
    let tx_slot = transaction.slot;
    let confirmations = current_slot.saturating_sub(tx_slot);

    if confirmations < REQUIRED_CONFIRMATIONS {
        return Err(StabuseError::InvalidData(
            "Insufficient transaction confirmations".to_string(),
        ));
    }

    let tx_meta = transaction
        .transaction
        .meta
        .ok_or_else(|| StabuseError::InvalidData("No transaction metadata".to_string()))?;

    if !tx_meta.status.is_ok() {
        return Err(StabuseError::InvalidData(
            "Transaction execution failed".to_string(),
        ));
    }

    let decoded_transaction = transaction.transaction.transaction.decode().unwrap();

    validate_transfer_instruction(
        pool,
        &decoded_transaction.into_legacy_transaction().unwrap(),
        &pending_payment,
        &pending_payment.asset,
        chain_id,
    )
    .await?;

    sqlx::query(DELETE_PENDING_PAYMENT)
        .bind(pending_payment_id)
        .execute(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    let id = sqlx::query_scalar(ADD_PAYMENT)
        .bind(pending_payment.merchant_id)
        .bind(pending_payment.sender)
        .bind(pending_payment.amount.to_string())
        .bind(tx_hash)
        .bind(pending_payment.asset)
        .bind(network)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok((id, pending_payment.webhook_url))
}

async fn validate_transfer_instruction(
    pool: &PgPool,
    transaction: &Transaction,
    pending_payment: &PendingPayment,
    token_mint: &str,
    chain_id: i64,
) -> Result<(), StabuseError> {
    let merchant_address =
        get_merchant_network_address(pool, pending_payment.merchant_id, chain_id)
            .await
            .map_err(|e| StabuseError::Internal(format!("Failed to get merchant address {}", e)))?;

    let token_mint_pubkey = Pubkey::from_str(token_mint)
        .map_err(|e| StabuseError::Internal(format!("Failed to get token mint: {}", e)))?;
    let merchant_pubkey = Pubkey::from_str(&merchant_address)
        .map_err(|e| StabuseError::Internal(format!("Failed to get merchant pubkey: {}", e)))?;
    let payer_pubkey = Pubkey::from_str(&pending_payment.sender)
        .map_err(|e| StabuseError::Internal(format!("Failed to get payer pubkey: {}", e)))?;

    let payer_token_account = get_associated_token_address(&payer_pubkey, &token_mint_pubkey);
    let merchant_token_account = get_associated_token_address(&merchant_pubkey, &token_mint_pubkey);

    let matching_instruction = transaction.message.instructions.iter().find(|instruction| {
        let program_ids: Vec<Pubkey> = transaction
            .message
            .program_ids()
            .into_iter()
            .cloned()
            .collect();

        let instruction_program_id = instruction.program_id(&program_ids);

        if *instruction_program_id != spl_token::id() {
            return false;
        }

        if let Ok(token_instruction) = TokenInstruction::unpack(&instruction.data) {
            match token_instruction {
                TokenInstruction::TransferChecked { amount, decimals } => {
                    let token_decimals = get_token_decimals(&pending_payment.asset).unwrap_or(0);

                    // Use direct comparison of Pubkeys instead of strings
                    instruction.accounts.len() >= 4
                        && Pubkey::from_str(&instruction.accounts[0].to_string()).unwrap()
                            == payer_token_account
                        && Pubkey::from_str(&instruction.accounts[1].to_string()).unwrap()
                            == token_mint_pubkey
                        && Pubkey::from_str(&instruction.accounts[2].to_string()).unwrap()
                            == merchant_token_account
                        && Pubkey::from_str(&instruction.accounts[3].to_string()).unwrap()
                            == payer_pubkey
                        && amount == pending_payment.amount.to_u64().unwrap()
                        && decimals == token_decimals
                }
                _ => false,
            }
        } else {
            false
        }
    });

    matching_instruction
        .ok_or_else(|| {
            StabuseError::InvalidData("No matching transfer instruction found".to_string())
        })
        .map(|_| ())
}
