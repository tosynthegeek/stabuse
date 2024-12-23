pub const CREATE_NETWORK_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS networks (
    id SERIAL PRIMARY KEY,
    chain_id BIGINT UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    rpc TEXT NOT NULL,
    supported_assets JSONB,
    last_updated_by VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)"#;
