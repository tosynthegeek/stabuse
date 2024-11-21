pub const ADD_MERCHANT: &str = r#"
    INSERT INTO merchants 
    (username, email, password_hash, supported_networks)
    VALUES ($1, $2, $3, $4)
    returning id;
"#;

pub const _UPDATE_MERCHANT_USERNAME: &str = r#"
    UPDATE merchants
    SET username = $2
    WHERE id = $1
    RETURNING id;
"#;

pub const _UPDATE_MERCHANT_EMAIL: &str = r#"
    UPDATE merchants
    SET email = $2
    WHERE id = $1
    RETURNING id;
"#;

pub const _UPDATE_MERCHANT_PASSWORD: &str = r#"
    UPDATE merchants
    SET password_hash = $2
    WHERE id = $1
    RETURNING id;
"#;

// pub const ADD_MERCHANT_ASSET: &str = r#"
//     UPDATE merchants
//     SET supported_assets =
//         CASE
//             WHEN supported_assets IS NULL THEN
//                 jsonb_build_object($2, $3)
//              ELSE
//                 supported_assets || jsonb_build_object($2, $3)
//         END
//     WHERE username = $1
//     RETURNING id;
// "#;

pub const ADD_MERCHANT_SUPPORTED_NETWORK: &str = r#"
    UPDATE merchants
    SET supported_networks = 
        CASE
            WHEN supported_networks IS NULL THEN
                jsonb_build_object(
                    $2::bigint::text,
                    jsonb_build_object(
                        'accepted_assets', to_jsonb($3::text[]),
                        'address', $4
                    )
                )
            ELSE
                jsonb_set(
                    supported_networks,
                    ARRAY[$2::bigint::text],
                    jsonb_build_object(
                        'accepted_assets', to_jsonb($3::text[]),
                        'address', $4
                    ),
                    true
                )
        END
    WHERE id = $1
    RETURNING supported_networks;
"#;

pub const ADD_ASSET_MERCHANT: &str = r#"
    UPDATE merchants
    SET supported_networks = jsonb_set(
        supported_networks,
        ARRAY[$1::bigint::text, 'accepted_assets'],
        (
            SELECT jsonb_agg(DISTINCT elem)
            FROM (
                SELECT elem
                FROM jsonb_array_elements_text(COALESCE(supported_networks->$1::bigint::text->'accepted_assets', '[]'::jsonb)) elem
                UNION
                SELECT $2::text
            ) subquery
        ),
        true
    )
    WHERE id = $3
    RETURNING supported_networks;
"#;

pub const REMOVE_ASSET_MERCHANT: &str = r#"
    UPDATE merchants
    SET supported_networks = jsonb_set(
        supported_networks,
        ARRAY[$1::bigint::text, 'accepted_assets'],
        (
            SELECT jsonb_agg(elem)
            FROM jsonb_array_elements(COALESCE(supported_networks->$1::bigint::text->'accepted_assets', '[]'::jsonb)) elem
            WHERE elem != to_jsonb($2::text)
        ),
        true
    )
    WHERE id = $3
    RETURNING supported_networks;
"#;

pub const UPDATE_NETWORK_ADDRESS_MERCHANT: &str = r#"
    UPDATE merchants
    SET supported_networks = jsonb_set(
        supported_networks,
        ARRAY[$1::text, 'address'],
        to_jsonb($2::text),
        true
    )
    WHERE id = $3
    RETURNING supported_networks;
"#;
