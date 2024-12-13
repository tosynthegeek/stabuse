pub const CREATE_PAYMENTS_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS payments (
    id SERIAL PRIMARY KEY,
    merchant_id INT REFERENCES merchants(id) ON DELETE CASCADE,
    sender VARCHAR(255) NOT NULL,
    amount NUMERIC(28,8) NOT NULL CHECK (amount > 0),
    tx_hash VARCHAR(255) UNIQUE NOT NULL,
    asset VARCHAR(255) NOT NULL,
    network VARCHAR(255) NOT NULL,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)"#;

pub const CREATE_PENDING_PAYMENTS_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS pending_payments (
    id SERIAL PRIMARY KEY,
    merchant_id INT REFERENCES merchants(id) ON DELETE CASCADE,
    sender VARCHAR(255) NOT NULL,
    amount NUMERIC(28,8) NOT NULL CHECK (amount > 0),
    asset VARCHAR(255) NOT NULL,
    network VARCHAR(255) NOT NULL,
    webhook_url TEXT NOT NULL,
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)"#;
