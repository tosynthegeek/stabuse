use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use rand::RngCore;

pub fn generate_secret() -> String {
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    BASE64_URL_SAFE_NO_PAD.encode(&key)
}

pub fn get_token_decimals(asset: &str) -> Result<u32, StabuseError> {
    TOKEN_DECIMALS
        .iter()
        .find(|(token, _)| *token == asset)
        .map(|(_, decimals)| *decimals)
        .ok_or_else(|| StabuseError::InvalidData(format!("Unsupported token: {}", asset)))
}
