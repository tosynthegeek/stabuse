use solana_client::rpc_client::RpcClient;
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer_checked;
use sqlx::{PgPool, Row};
use std::{env, str::FromStr};

use crate::{
    auth::jwt::generate_payment_jwt,
    db::migrations::payments::inserts_and_updates::ADD_PENDING_PAYMENT,
    merchant::merchant::get_merchant_network_address,
    network::network::get_network_and_asset_address_with_chain_id,
    utils::utils::get_solana_network_identifier,
};

pub async fn create_payment_transaction(
    pool: &PgPool,
    rpc_url: &str,
    payer: &str,
    merchant_id: i32,
    asset: &str,
    amount: u64,
    decimals: u8,
) -> Result<(Transaction, String), Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new(rpc_url.to_string());
    let chain_id = get_solana_network_identifier(rpc_url)?;
    let merchant = get_merchant_network_address(pool, merchant_id, chain_id).await?;
    let (network, token_mint) =
        get_network_and_asset_address_with_chain_id(pool, asset, chain_id as u64).await?;
    let payer_pubkey = Pubkey::from_str(payer)?;
    let merchant_pubkey = Pubkey::from_str(merchant.as_str())?;
    let token_mint_pubkey = Pubkey::from_str(token_mint.as_str())?;

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

    let pending_payment_id: i32 = sqlx::query(ADD_PENDING_PAYMENT)
        .bind(merchant_id)
        .bind(payer)
        .bind(amount.to_string())
        .bind(asset)
        .bind(network)
        .fetch_one(pool)
        .await?
        .get(0);

    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let token = generate_payment_jwt(pending_payment_id, &jwt_secret)?;

    Ok((transaction, token))
}
