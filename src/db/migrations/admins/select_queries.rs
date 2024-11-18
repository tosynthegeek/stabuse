pub const LOGIN_ATTEMPT: &str = r#"
    SELECT id, username, password_hash 
    FROM admins 
    WHERE username = $1 
       OR email = $1
"#;

pub const GET_INVITE_DETAILS: &str = r#"
    SELECT email, token, expires_at 
    FROM admin_invites 
    WHERE email = $1
"#;

pub const GET_ADMIN_COUNT: &str = r#"
    SELECT COUNT(*) FROM admins
"#;
