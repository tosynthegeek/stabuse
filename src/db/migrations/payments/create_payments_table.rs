pub const CREATE_PAYMENTS_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS payments (
    id SERIAL PRIMARY KEY,
    merchant_id INT REFERENCES merchants(id) ON DELETE CASCADE,
    sender VARCHAR(255) NOT NULL,
    amount NUMERIC NOT NULL,
    tx_hash VARCHAR(255) UNIQUE NOT NULL,
    asset VARCHAR(255) NOT NULL,
    network VARCHAR(255) NOT NULL,
    time TIMESTAMP NOT NULL
)"#;
