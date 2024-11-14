use std::collections::HashMap;

use crate::error::StabuseError;

pub fn validate_assets(assets: &HashMap<String, String>) -> Result<(), StabuseError> {
    for (ticker, address) in assets {
        if ticker == "" {
            return Err(StabuseError::InvalidAssetFormat(format!("Ticker cannot be empty")));
        }
        if address.len() != 42 || !address.starts_with("0x") {
            return Err(StabuseError::InvalidAssetFormat(format!(
                "Invalid address format for asset {}: {}",
                ticker, address
            )));
        }
    }
    Ok(())
}
