pub const ADD_ADMIN: &str = r#"
    INSERT INTO admins 
    (email, username, password_hash, is_super_admin)
    VALUES ($1, $2, $3, false)
    returning id;
"#;

pub const ADD_SUPER_ADMIN: &str = r#"
    INSERT INTO admins 
    (email, username, password_hash, is_super_admin)
    VALUES ($1, $2, $3, true)
    returning id;
"#;

pub const ADD_ADMIN_INVITE: &str = r#"
    INSERT INTO admin_invites 
    (email, token, expires_at) 
    VALUES ($1, $2, $3)
    returning id
"#;

pub const DELETE_ADMIN_INVITE: &str = r#"
    DELETE FROM admin_invites 
    WHERE email = $1
"#;

pub const ADD_OTP: &str = r#"
    INSERT INTO admin_otps (email, otp_hash, expires_at)
    VALUES ($1, $2, $3)
    ON CONFLICT (email) DO UPDATE 
    SET otp_hash = $2,
        expires_at = $3
    RETURNING id
"#;

pub const DELETE_OTP: &str = r#"
    DELETE FROM admin_otps WHERE email = $1
"#;

/*
TODO! Added admins tasks/priviledges
Blacklisting merchants
Deleting merchants
*/
