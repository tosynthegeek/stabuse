pub const LOGIN_ATTEMPT: &str = r#"
    SELECT id, email, username, password_hash 
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

pub const _IS_ADMIN: &str = r#"
    SELECT EXISTS (SELECT 1 FROM admin_table WHERE email = $1)
"#;

pub const GET_OTP: &str = r#"
    SELECT otp_hash, expires_at 
    FROM admin_otps WHERE email = $1
"#;

pub const _GET_ADMIN: &str = r#"
    SELECT id 
    FROM admins 
    WHERE email = $1
"#;
