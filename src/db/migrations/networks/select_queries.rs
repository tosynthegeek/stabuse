pub const GET_NETWORK_ASSETS: &str = r#"
    SELECT supported_assets FROM networks
    WHERE chain_id = $1
"#;

pub const GET_NETWORK: &str = r#"
    SELECT * FROM networks
    WHERE chain_id = $1
"#;

pub const GET_ALL_NETWORKS: &str = r#"
    SELECT * FROM networks
"#;

pub const CHECK_NETWORK_SUPPORTED_ASSET: &str = r#"
    SELECT EXISTS(
        SELECT 1
        FROM networks
        WHERE chain_id = $1
          AND supported_assets ? $2
    )
"#;
