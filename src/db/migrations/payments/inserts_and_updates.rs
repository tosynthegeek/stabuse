pub const ADD_PAYMENT: &str = r#"
    INSERT INTO payments 
        (merchant_id, sender, amount, tx_hash, asset, network)
    VALUES 
        ($1, $2, $3::NUMERIC, $4, $5, $6)
    returning id
"#;

pub const ADD_PENDING_PAYMENT: &str = r#"
    INSERT INTO pending_payments 
        (merchant_id, sender, amount, asset, network)
    VALUES 
        ($1, $2, $3::NUMERIC, $4, $5)
    returning id
"#;

pub const DELETE_PENDING_PAYMENT: &str = r#"
    DELETE FROM pending_payments
    WHERE id = $1
"#;
