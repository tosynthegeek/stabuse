pub const CREATE_INDEX_USDT: &str =
    r#"CREATE INDEX IF NOT EXISTS idx_networks_usdt ON networks((supported_assets->>'USDT'));"#;
pub const CREATE_INDEX_DAI: &str =
    r#"CREATE INDEX IF NOT EXISTS idx_networks_dai ON networks((supported_assets->>'DAI'));"#;
pub const CREATE_INDEX_USDC: &str =
    r#"CREATE INDEX IF NOT EXISTS idx_networks_usdc ON networks((supported_assets->>'USDC'));"#;
pub const CREATE_INDEX_BUSD: &str =
    r#"CREATE INDEX IF NOT EXISTS idx_networks_busd ON networks((supported_assets->>'BUSD'));"#;
