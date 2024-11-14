use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::error::StabuseError;

use super::migrations::{CREATE_INDEX_BUSD, CREATE_INDEX_DAI, CREATE_INDEX_USDC, CREATE_INDEX_USDT,  CREATE_NETWORK_TABLE, TRIGGER, TRIGGER_FUNCTION};

pub async fn init_db(pool: &PgPool) -> Result<(), StabuseError> {
    sqlx::query(CREATE_NETWORK_TABLE).execute(pool).await?;
    sqlx::query(TRIGGER).execute(pool).await?;
    sqlx::query(TRIGGER_FUNCTION).execute(pool).await?;
    sqlx::query(CREATE_INDEX_USDT).execute(pool).await?;
    sqlx::query(CREATE_INDEX_DAI).execute(pool).await?;
    sqlx::query(CREATE_INDEX_USDC).execute(pool).await?;
    sqlx::query(CREATE_INDEX_BUSD).execute(pool).await?;

    Ok(())
}

pub async fn connect_db() -> Result<Pool<Postgres>, StabuseError> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
                                        .max_connections(5)
                                        .connect(&database_url)
                                        .await
                                        .map_err(|e| {
                                            StabuseError::DatabaseError(e)
                                        })?;
    println!("Connected to the database!");

    Ok(pool)

}
