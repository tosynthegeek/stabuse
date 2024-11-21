pub const LOGIN_ATTEMPT: &str = r#"
    SELECT id, username, password_hash 
    FROM merchants 
    WHERE username = $1 
       OR email = $1
"#;

pub const GET_MERCHANT: &str = r#"
    SELECT username, supported_networks 
    FROM merchants
    WHERE id = $1
"#;
