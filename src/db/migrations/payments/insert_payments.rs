pub const ADD_PAYMENT: &str = r#"
    INSERT INTO payments 
        (merchant_id, sender, amount, tx_hash, asset, network, time)
    VALUES 
        ($1, $2, $3, $4, $5, $6, $7)
)"#;

