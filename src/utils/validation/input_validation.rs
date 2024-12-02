use regex::Regex;

use crate::{
    error::StabuseError,
    utils::utils::{MIN_PASSWORD_LENGTH, MIN_USERNAME_LENGTH},
};

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).expect("Invalid regex pattern");

    email_regex.is_match(email)
}

pub fn validate_password(password: &str) -> Result<(), StabuseError> {
    if password.is_empty() {
        return Err(StabuseError::InvalidCredentials(
            "Password cannot be empty".into(),
        ));
    }
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(StabuseError::InvalidCredentials(format!(
            "Password must be at least {} characters long",
            MIN_PASSWORD_LENGTH
        )));
    }
    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), StabuseError> {
    if username.is_empty() {
        return Err(StabuseError::InvalidCredentials(
            "Username cannot be empty".into(),
        ));
    }
    if username.len() < MIN_USERNAME_LENGTH {
        return Err(StabuseError::InvalidCredentials(format!(
            "Username must be at least {} characters long",
            MIN_USERNAME_LENGTH
        )));
    }

    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(StabuseError::InvalidCredentials(
            "Username can only contain alphanumeric characters, underscores, and hyphens".into(),
        ));
    }
    Ok(())
}
