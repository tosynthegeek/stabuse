pub const CREATE_NETWORK_TABLE: &str = r#"
    CREATE TABLE IF NOT EXISTS networks (
    id SERIAL PRIMARY KEY,
    chain_id BIGINT UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    rpc TEXT NOT NULL,
    supported_assets JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)"#;

pub const TRIGGER: &str = r#"
    CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS $$
    BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
    END;
    $$ LANGUAGE plpgsql
"#;

pub const TRIGGER_FUNCTION: &str = r#" 
    CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON networks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
"#;

pub const CREATE_INDEX_USDT: &str = r#"CREATE INDEX IF NOT EXISTS idx_networks_usdt ON networks((supported_assets->>'USDT'));"#;
pub const CREATE_INDEX_DAI: &str = r#"CREATE INDEX IF NOT EXISTS idx_networks_dai ON networks((supported_assets->>'DAI'));"#;
pub const CREATE_INDEX_USDC: &str = r#"CREATE INDEX IF NOT EXISTS idx_networks_usdc ON networks((supported_assets->>'USDC'));"#;
pub const CREATE_INDEX_BUSD: &str = r#"CREATE INDEX IF NOT EXISTS idx_networks_busd ON networks((supported_assets->>'BUSD'));"#;

pub const ADD_NETWORK: &str = r#"
    INSERT INTO networks 
    (chain_id, name, rpc, supported_assets)
    VALUES ($1, $2, $3, $4)
    returning id;
"#;

pub const ADD_ASSET: &str = r#"
    UPDATE networks
    SET supported_assets = 
        CASE
            WHEN supported_assets IS NULL THEN
                '{"$1": "$2"}'::jsonb
            ELSE
                supported_assets || jsonb_build_object($1, $2)
        END
    WHERE chain_id = $3;
"#;

pub const GET_ASSET_ADDRESS: &str = r#"
    SELECT FROM networks

"#;

pub const GET_ASSETS: &str = r#"
    SELECT supported_assets FROM networks
    WHERE chain_id = $1
"#;