use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::error::StabuseError;

use super::migrations::{
    admins::create_admins_table::{
        CREATE_ADMINS_TABLE, CREATE_ADMIN_INVITES_TABLE, CREATE_OTP_TABLE,
    },
    merchants::{
        create_merchants_table::CREATE_MERCHANT_TABLE, triggers::TRIGGER_FUNCTION_MERCHANTS,
    },
    networks::{
        create_indexes::{
            CREATE_INDEX_BUSD, CREATE_INDEX_DAI, CREATE_INDEX_USDC, CREATE_INDEX_USDT,
        },
        create_networks_table::CREATE_NETWORK_TABLE,
        triggers_and_functions::{TRIGGER, TRIGGER_FUNCTION},
    },
    payments::{
        create_indexes::{CREATE_INDEX_MERCHANT_ID, CREATE_INDEX_NETWORK, CREATE_INDEX_TX_HASH},
        create_payments_table::{CREATE_PAYMENTS_TABLE, CREATE_PENDING_PAYMENTS_TABLE},
    },
};

pub async fn init_db(pool: &PgPool) -> Result<(), StabuseError> {
    sqlx::query(CREATE_NETWORK_TABLE).execute(pool).await?;
    sqlx::query(CREATE_MERCHANT_TABLE).execute(pool).await?;
    sqlx::query(CREATE_PAYMENTS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_PENDING_PAYMENTS_TABLE)
        .execute(pool)
        .await?;
    sqlx::query(TRIGGER).execute(pool).await?;
    sqlx::query(TRIGGER_FUNCTION).execute(pool).await?;
    sqlx::query(TRIGGER_FUNCTION_MERCHANTS)
        .execute(pool)
        .await?;
    sqlx::query(CREATE_INDEX_USDT).execute(pool).await?;
    sqlx::query(CREATE_INDEX_DAI).execute(pool).await?;
    sqlx::query(CREATE_INDEX_USDC).execute(pool).await?;
    sqlx::query(CREATE_INDEX_BUSD).execute(pool).await?;
    sqlx::query(CREATE_INDEX_MERCHANT_ID).execute(pool).await?;
    sqlx::query(CREATE_INDEX_NETWORK).execute(pool).await?;
    sqlx::query(CREATE_INDEX_TX_HASH).execute(pool).await?;
    sqlx::query(CREATE_ADMINS_TABLE).execute(pool).await?;
    sqlx::query(CREATE_ADMIN_INVITES_TABLE)
        .execute(pool)
        .await?;
    sqlx::query(CREATE_OTP_TABLE).execute(pool).await?;

    Ok(())
}

pub async fn connect_db() -> Result<Pool<Postgres>, StabuseError> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;
    println!("Connected to the database!");

    Ok(pool)
}
