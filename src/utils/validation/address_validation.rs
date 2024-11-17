use crate::error::StabuseError;

pub fn validate_address(address: &str) -> Result<(), StabuseError> {
    if address.len() != 42 || !address.starts_with("0x") {
        return Err(StabuseError::InvalidData(format!(
            "Invalid address format for Network {}",
            address
        )));
    }
    Ok(())
}