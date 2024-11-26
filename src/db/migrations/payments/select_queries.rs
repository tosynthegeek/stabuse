pub const _GET_PAYMENTS_FOR_MERCHANT: &str = r#"
    SELECT id, sender, amount, tx_hash, asset, network, time 
    FROM payments
    WHERE merchant_id = $1
    ORDER BY time DESC
    LIMIT $2 OFFSET $3
"#;

pub const GET_PENDING_PAYMENT: &str = r#"
    SELECT id, merchant_id, sender, amount, asset, network, time 
    FROM pending_payments
    WHERE id = $1
"#;

pub const _GET_PAYMENT_BY_TX_HASH: &str = r#"
    SELECT id, merchant_id, sender, amount, asset, network, time 
    FROM payments
    WHERE tx_hash = $1
"#;

pub const _COUNT_PAYMENTS_FOR_MERCHANT: &str = r#"
    SELECT COUNT(*)
    FROM payments
    WHERE merchant_id = $1
"#;

pub const _SEARCH_MERCHANT_PAYMENTS_BY_SENDER: &str = r#"
    SELECT id, amount, tx_hash, asset, network, time 
    FROM payments
    WHERE merchant_id = $1 AND sender ILIKE $2
    ORDER BY time DESC
"#;

pub const _GET_PAYMENT_BY_ID: &str = r#"
    SELECT id, merchant_id, sender, amount, tx_hash, asset, network, time 
    FROM payments
    WHERE id = $1
"#;

pub const _AGGREGATE_PAYMENTS: &str = r#"
    SELECT network, asset, SUM(amount) AS total_amount 
    FROM payments
    GROUP BY network, asset
    ORDER BY total_amount DESC
"#;
