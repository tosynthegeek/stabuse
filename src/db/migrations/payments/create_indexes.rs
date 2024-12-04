pub const CREATE_INDEX_MERCHANT_ID: &str = r#"
    CREATE INDEX idx_payments_merchant_id ON payments (merchant_id)"#;
pub const CREATE_INDEX_TX_HASH: &str = r#"
    CREATE INDEX idx_payments_tx_hash ON payments (tx_hash)"#;
pub const CREATE_INDEX_NETWORK: &str = r#"
    CREATE INDEX idx_payments_network ON payments (network)"#;
