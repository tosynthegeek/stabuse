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
