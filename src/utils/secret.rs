use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use rand::RngCore;

pub fn generate_secret() -> String {
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    BASE64_URL_SAFE_NO_PAD.encode(&key)
}
