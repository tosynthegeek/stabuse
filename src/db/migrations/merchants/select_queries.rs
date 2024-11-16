pub const _GET_PASSWORD_HASH: &str = r#"
    SELECT password_hash 
    FROM merchant 
    WHERE username = $1 
       OR email = $1
"#;
