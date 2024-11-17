pub const LOGIN_ATTEMPT: &str = r#"
    SELECT id, username, password_hash 
    FROM merchants 
    WHERE username = $1 
       OR email = $1
"#;
