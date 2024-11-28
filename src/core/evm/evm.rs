use alloy::{
    consensus::Transaction,
    eips::BlockNumberOrTag,
    hex,
    primitives::{keccak256, Address, FixedBytes, TxKind, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{TransactionInput, TransactionReceipt, TransactionRequest},
};
use alloy_sol_types::{sol, SolCall};
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
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
    types::types::{CreatePaymentTransaction, PendingPayment, TransactionValidationParams},
    utils::{utils::get_token_decimals, validation::address_validation::validate_address},
};

const REQUIRED_CONFIRMATIONS: u64 = 12;

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    interface IERC20 {
        function transfer(address to, uint256 value) external returns (bool);
        event Transfer(address indexed from, address indexed to, uint256 value);
    }
}

pub async fn create_payment_request(
    pool: &PgPool,
    merchant_id: i32,
    amount: u64,
    user_address: &str,
    rpc_url: &str,
    asset: &str,
) -> Result<(CreatePaymentTransaction, String), StabuseError> {
    validate_address(user_address)?;

    let rpc = rpc_url
        .parse()
        .map_err(|e| StabuseError::Internal(format!("Invalid RPC URL: {}", e)))?;
    let provider = ProviderBuilder::new().on_http(rpc);
    let chain_id = provider.get_chain_id().await?;

    let merchant_address =
        get_merchant_network_address(pool, merchant_id, chain_id.try_into().unwrap()).await?;
    let (network, token_address) =
        get_network_and_asset_address_with_chain_id(pool, asset, chain_id).await?;
    let from_address = Address::from_str(user_address)
        .map_err(|e| StabuseError::Internal(format!("Invalid user address: {}", e)))?;
    let to_address = Address::from_str(&merchant_address)
        .map_err(|e| StabuseError::Internal(format!("Invalid merchant address: {}", e)))?;

    let transfer_call = IERC20::transferCall {
        to: to_address,
        value: U256::from(amount),
    };
    let call_data = transfer_call.abi_encode();
    println!("Token Address: {}", token_address);

    let nonce = provider.get_transaction_count(from_address).await?;

    let gas_estimate = {
        let tx = TransactionRequest {
            from: Some(from_address),
            to: Some(TxKind::from(Address::from_str(&token_address).map_err(
                |e| StabuseError::Internal(format!("Invalid token address: {}", e)),
            )?)),
            input: Some(TransactionInput {
                input: None,
                data: Some(call_data.clone().into()),
            })
            .unwrap(),
            ..Default::default()
        };
        provider.estimate_gas(&tx).await?
    };

    let fee_history = provider
        .get_fee_history(1, Some(BlockNumberOrTag::Latest).unwrap(), &[])
        .await?;

    let (max_fee_per_gas, max_priority_fee_per_gas) =
        if let Some(base_fee) = fee_history.base_fee_per_gas.first() {
            let priority_fee = U256::from(1_000_000_000);
            let max_fee = U256::from(*base_fee) + priority_fee;
            (Some(max_fee), Some(priority_fee))
        } else {
            (None, None)
        };

    let pending_payment_id: i32 = sqlx::query(ADD_PENDING_PAYMENT)
        .bind(merchant_id)
        .bind(user_address)
        .bind(amount.to_string())
        .bind(asset)
        .bind(network)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?
        .get(0);

    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let token = generate_payment_jwt(pending_payment_id, &jwt_secret)?;

    Ok((
        CreatePaymentTransaction {
            to: token_address,
            from: user_address.to_string(),
            data: hex::encode(call_data),
            value: "0x0".to_string(),
            nonce: format!("0x{:x}", nonce),
            chain_id: chain_id.try_into().unwrap(),
            gas_limit: Some(format!("0x{:x}", gas_estimate)),
            max_fee_per_gas: max_fee_per_gas.map(|f| format!("0x{:x}", f)),
            max_priority_fee_per_gas: max_priority_fee_per_gas.map(|f| format!("0x{:x}", f)),
        },
        token,
    ))
}

pub async fn verify_signed_transaction(
    pool: &PgPool,
    pending_payment_id: i32,
    rpc_url: &str,
    tx_hash: &str,
) -> Result<i32, StabuseError> {
    let rpc = rpc_url
        .parse()
        .map_err(|e| StabuseError::Internal(format!("Invalid RPC URL: {}", e)))?;
    let provider = ProviderBuilder::new().on_http(rpc);
    let chain_id = provider.get_chain_id().await?;
    println!("Pending payment id: {}", pending_payment_id);
    let pending_payment = sqlx::query_as::<_, PendingPayment>(GET_PENDING_PAYMENT)
        .bind(pending_payment_id)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;
    let (network, token_address) =
        get_network_and_asset_address_with_chain_id(pool, &pending_payment.asset, chain_id).await?;

    let tx_hash_bytes = hex::decode(tx_hash.trim_start_matches("0x"))
        .map_err(|_| StabuseError::InvalidData("Invalid transaction hash".to_string()))?;

    let tx_hash_array: [u8; 32] = tx_hash_bytes
        .try_into()
        .map_err(|_| StabuseError::InvalidData("Invalid transaction hash length".to_string()))?;

    let tx_hash_fixed: FixedBytes<32> = FixedBytes::from(tx_hash_array);

    let receipt = provider
        .get_transaction_receipt(tx_hash_fixed)
        .await
        .map_err(|e| {
            StabuseError::Internal(format!("Failed to fetch transaction receipt: {}", e))
        })?;

    let receipt = match receipt {
        Some(receipt) => receipt,
        None => {
            return Err(StabuseError::Internal(format!(
                "Failed to fetch transaction receipt"
            )))
        }
    };

    while let Some(tx_block_number) = receipt.block_number {
        let current_block = provider.get_block_number().await?;
        let confirmations = current_block.saturating_sub(tx_block_number);

        if confirmations >= REQUIRED_CONFIRMATIONS {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    let decimal = get_token_decimals(&pending_payment.asset)?;
    let amount = pending_payment
        .amount
        .to_f64()
        .ok_or_else(|| StabuseError::Internal("Invalid amount".to_string()))?;
    let converted_amount = (amount * 10f64.powi(decimal as i32)) as u128;

    let tx = provider
        .get_transaction_by_hash(tx_hash_fixed)
        .await?
        .ok_or_else(|| StabuseError::Internal("Transaction not found".to_string()))?;

    let merchant_address = get_merchant_network_address(
        pool,
        pending_payment.merchant_id,
        chain_id.try_into().unwrap(),
    )
    .await?;

    let expected_to = Address::from_str(&token_address)
        .map_err(|e| StabuseError::Internal(format!("Invalid token address: {}", e)))?;
    let expected_data = IERC20::transferCall {
        to: Address::from_str(&merchant_address)
            .map_err(|e| StabuseError::Internal(format!("Invalid merchant address: {}", e)))?,
        value: U256::from(converted_amount),
    }
    .abi_encode();

    let tx_inner = tx.inner;
    let tx_kind = tx_inner.kind();

    match tx_kind {
        TxKind::Call(to) => {
            if to != expected_to {
                return Err(StabuseError::InvalidData(
                    "Transaction recipient (to) address does not match.".to_string(),
                ));
            }
        }
        _ => {
            return Err(StabuseError::InvalidData(
                "Unsupported transaction kind.".to_string(),
            ))
        }
    }

    let tx_data = tx_inner.input();

    if tx_data.to_vec() != expected_data {
        return Err(StabuseError::InvalidData(
            "Transaction data does not match the expected data.".to_string(),
        ));
    }

    if !receipt.status() {
        return Err(StabuseError::InvalidData(
            "Transaction execution failed.".to_string(),
        ));
    }

    let validation_params = TransactionValidationParams {
        merchant_address: get_merchant_network_address(
            pool,
            pending_payment.merchant_id,
            chain_id.try_into().unwrap(),
        )
        .await?
        .parse()
        .map_err(|e| StabuseError::Internal(format!("Invalid merchant address: {}", e)))?,
        token_address: token_address
            .parse()
            .map_err(|e| StabuseError::Internal(format!("Invalid token address: {}", e)))?,
        user_address: Address::from_str(&pending_payment.sender)
            .map_err(|e| StabuseError::Internal(format!("Invalid user address: {}", e)))?,
        amount: U256::from(converted_amount),
    };

    validate_transfer_event(&receipt, &validation_params)?;

    sqlx::query(DELETE_PENDING_PAYMENT)
        .bind(pending_payment_id)
        .execute(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    println!("Pending payment: {:?}", pending_payment);
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

    Ok(id)
}

fn validate_transfer_event(
    receipt: &TransactionReceipt,
    params: &TransactionValidationParams,
) -> Result<(), StabuseError> {
    let transfer_signature = keccak256("Transfer(address,address,uint256)");

    let total_transfer_amount: U256 = receipt
        .inner
        .as_receipt()
        .ok_or(StabuseError::InvalidData("Invalid receipt".to_string()))?
        .logs
        .iter()
        .filter_map(|log| {
            if log.inner.address != params.token_address {
                return None;
            }
            if log.inner.topics().get(0) != Some(&transfer_signature) {
                return None;
            }

            let from = log
                .inner
                .topics()
                .get(1)
                .and_then(|topic| Some(Address::from_slice(&topic.0[12..])));
            let to = log
                .inner
                .topics()
                .get(2)
                .and_then(|topic| Some(Address::from_slice(&topic.0[12..])));

            if from != Some(params.user_address) || to != Some(params.merchant_address) {
                return None;
            }

            U256::try_from_be_slice(&log.inner.data.data.0)
        })
        .sum();

    println!("total amount: {}", total_transfer_amount);
    println!("expected amount: {}", params.amount);
    if total_transfer_amount != params.amount {
        return Err(StabuseError::InvalidData(
            "Total transfer amount does not match expected amount".to_string(),
        ));
    }

    Ok(())
}
