use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use rand::RngCore;

pub fn generate_secret() -> String {
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    BASE64_URL_SAFE_NO_PAD.encode(&key)
}

pub fn get_network_and_asset_address(
    asset: &str,
    chain_id: u64,
) -> Result<(String, String), StabuseError> {
    let data = fs::read_to_string("config/assets.json")?;

    let assets: Value = serde_json::from_str(&data)?;

    if let Some(network_data) = assets.get(chain_id.to_string()) {
        let network_name = network_data
            .get("network")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                StabuseError::AssetNotSupportedonNetwork(format!(
                    "Network name not found for chain ID {}",
                    chain_id
                ))
            })?;

        let asset_address = network_data
            .get(asset)
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                StabuseError::AssetNotSupportedonNetwork(format!(
                    "Asset {} not found on network {}",
                    asset, chain_id
                ))
            })?;

        Ok((network_name.to_string(), asset_address.to_string()))
    } else {
        Err(StabuseError::AssetNotSupportedonNetwork(format!(
            "Chain ID {} not found in configuration",
            chain_id
        )))
    }
}
