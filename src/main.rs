mod admin;
mod auth;
mod core;
mod db;
mod error;
mod handlers;
mod merchant;
mod network;
mod routes;
mod types;
mod utils;

use actix_web::{web, App, HttpServer};
use db::db_init::connect_db;
use env_logger::Env;
use routes::routes::{
    configure_admin_routes, configure_merchant_api_routes, configure_public_routes,
};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let address: &str = "0.0.0.0:80";
    info!("Starting micrors at http://{}", address);

    let pool = connect_db().await.expect("erro conneting to db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) //uses Arc
            .configure(configure_public_routes)
            .configure(configure_merchant_api_routes)
            .configure(configure_admin_routes)
    })
    .bind(address)
    .unwrap()
    .run()
    .await
}
