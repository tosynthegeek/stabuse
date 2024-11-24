use alloy::{
    consensus::Transaction,
    eips::BlockNumberOrTag,
    hex,
    primitives::{Address, FixedBytes, TxKind, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{TransactionInput, TransactionRequest},
};
use alloy_sol_types::{sol, SolCall};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::str::FromStr;

use crate::{
    error::StabuseError,
    merchant::merchant::get_merchant_network_address,
    types::types::CreatePaymentTransaction,
    utils::{utils::get_asset_contract_address, validation::address_validation::validate_address},
};

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    interface IERC20 {
        function transfer(address to, uint256 value) external returns (bool);
    }
}

pub async fn create_payment_request(
    pool: &PgPool,
    merchant_id: i32,
    amount: u64,
    user_address: &str,
    rpc_url: &str,
    asset: &str,
) -> Result<CreatePaymentTransaction, StabuseError> {
    validate_address(user_address)?;

    let rpc = rpc_url
        .parse()
        .map_err(|e| StabuseError::Internal(format!("Invalid RPC URL: {}", e)))?;
    let provider = ProviderBuilder::new().on_http(rpc);
    let chain_id = provider.get_chain_id().await?;

    let merchant_address =
        get_merchant_network_address(pool, merchant_id, chain_id.try_into().unwrap()).await?;
    let token_address = get_asset_contract_address(asset, chain_id)?;
    let from_address = Address::from_str(user_address)
        .map_err(|e| StabuseError::Internal(format!("Invalid user address: {}", e)))?;
    let to_address = Address::from_str(&merchant_address)
        .map_err(|e| StabuseError::Internal(format!("Invalid merchant address: {}", e)))?;

    let transfer_call = IERC20::transferCall {
        to: to_address,
        value: U256::from(amount),
    };
    let call_data = transfer_call.abi_encode();

    let nonce = provider.get_transaction_count(from_address).await?;

    let gas_estimate = {
        let tx = TransactionRequest {
            from: Some(from_address),
            to: Some(TxKind::from(
                Address::from_str(&token_address)
                    .map_err(|e| StabuseError::Internal(e.to_string()))?,
            )),
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

    Ok(CreatePaymentTransaction {
        to: token_address,
        from: user_address.to_string(),
        data: hex::encode(call_data),
        value: "0x0".to_string(),
        nonce: format!("0x{:x}", nonce),
        chain_id: chain_id.try_into().unwrap(),
        gas_limit: Some(format!("0x{:x}", gas_estimate)),
        max_fee_per_gas: max_fee_per_gas.map(|f| format!("0x{:x}", f)),
        max_priority_fee_per_gas: max_priority_fee_per_gas.map(|f| format!("0x{:x}", f)),
    })
}

pub async fn verify_signed_transaction(
    pool: &PgPool,
    merchant_id: i32,
    amount: u64,
    user_address: &str,
    rpc_url: &str,
    asset: &str,
    tx_hash: &str,
) -> Result<i32, StabuseError> {
    let rpc = rpc_url
        .parse()
        .map_err(|e| StabuseError::Internal(format!("Invalid RPC URL: {}", e)))?;
    let provider = ProviderBuilder::new().on_http(rpc);

    let tx_hash_bytes = hex::decode(tx_hash.trim_start_matches("0x"))
        .map_err(|_| StabuseError::InvalidData("Invalid transaction hash".to_string()))?;

    let tx_hash_array: [u8; 32] = tx_hash_bytes
        .try_into()
        .map_err(|_| StabuseError::InvalidData("Invalid transaction hash length".to_string()))?;

    let tx_hash_fixed: FixedBytes<32> = FixedBytes::from(tx_hash_array);

    let tx = provider
        .get_transaction_by_hash(tx_hash_fixed)
        .await?
        .ok_or_else(|| StabuseError::Internal("Transaction not found".to_string()))?;

    let chain_id = provider.get_chain_id().await?;

    let merchant_address =
        get_merchant_network_address(pool, merchant_id, chain_id.try_into().unwrap()).await?;

    let token_address = get_asset_contract_address(asset, chain_id)?;

    let expected_to = Address::from_str(&token_address)
        .map_err(|e| StabuseError::Internal(format!("Invalid token address: {}", e)))?;
    let expected_data = IERC20::transferCall {
        to: Address::from_str(&merchant_address)
            .map_err(|e| StabuseError::Internal(format!("Invalid merchant address: {}", e)))?,
        value: U256::from(amount),
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

    let receipt = provider
        .get_transaction_receipt(tx_hash_fixed)
        .await
        .map_err(|e| {
            StabuseError::Internal(format!("Failed to fetch transaction receipt: {}", e))
        })?;

    let receipt = receipt
        .ok_or_else(|| StabuseError::InvalidData("Transaction receipt not found.".to_string()))?;
    if !receipt.status() {
        return Err(StabuseError::InvalidData(
            "Transaction execution failed.".to_string(),
        ));
    }

    Ok(merchant_id)
}
