use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_token::instruction::transfer_checked;

pub async fn create_payment_transaction(
    rpc_url: &str,
    payer_pubkey: &str,
    merchant_pubkey: &str,
    asset: &str,
    amount: u64,
    decimals: u8,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new(rpc_url.to_string());
    let merchant_pubkey = get_merchant_network_address(pool, merchant_id, SOLANA_CHAIN_ID).await?;
    let (network, token_mint) = get_network_and_asset_address(pool, asset, SOLANA_CHAIN_ID).await?;
    let payer = Pubkey::from_str(payer_pubkey)?;
    let merchant = Pubkey::from_str(merchant_pubkey)?;
    let token_mint_pubkey = Pubkey::from_str(token_mint)?;

    let payer_token_account =
        spl_associated_token_account::get_associated_token_address(&payer, &token_mint_pubkey);
    let merchant_token_account =
        spl_associated_token_account::get_associated_token_address(&merchant, &token_mint_pubkey);

    let transfer_instruction = transfer_checked(
        &spl_token::id(),
        &payer_token_account,
        &token_mint_pubkey,
        &merchant_token_account,
        &payer,
        &[],
        amount,
        decimals,
    )?;

    let message = Message::new(&[transfer_instruction], Some(&payer));
    let recent_blockhash = rpc_client.get_latest_blockhash()?;

    let transaction = Transaction::new_unsigned(message);
    Ok(transaction)
}
