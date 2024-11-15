use std::collections::HashMap;
use serde_json::{self, Value};
use crate::error::StabuseError;

pub fn transform_assets_to_uppercase(assets: &HashMap<String, String>) -> HashMap<String, String> {
    assets.iter()
        .map(|(ticker, address)| (ticker.to_uppercase(), address.clone()))
        .collect()
}

pub fn hashmap_to_json_value(map: HashMap<String, String>) -> Result<Value, StabuseError> {
    let value = serde_json::to_value(map).map_err(|err| StabuseError::SerdeError(err.to_string()))?;

    Ok(value)
}
