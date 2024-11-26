pub const LOGIN_ATTEMPT: &str = r#"
    SELECT id, username, password_hash 
    FROM merchants 
    WHERE username = $1 
       OR email = $1
"#;

pub const _GET_MERCHANT: &str = r#"
    SELECT username, supported_networks 
    FROM merchants
    WHERE id = $1
"#;

pub const GET_MERCHANT_NETWORK_ADDRESS: &str = r#"
    SELECT 
        supported_networks -> $2::text ->> 'address' AS address
    FROM merchants
    WHERE id = $1;
"#;
