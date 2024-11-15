use std::collections::HashMap;
use regex::Regex;

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

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?
          (?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$",
    )
    .unwrap();

    email_regex.is_match(email)
}

