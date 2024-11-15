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