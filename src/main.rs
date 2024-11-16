mod db;
mod error;
mod handlers;
mod merchant;
mod network;
mod types;
mod utils;

use actix_web::{web, App, HttpServer};
use db::db_init::connect_db;
use env_logger::Env;
use handlers::{
    handle_init_bd,
    merchant_handlers::{
        add_merchant_asset_handler, add_merchant_network_handler, create_merchant_account_handler,
        remove_merchant_asset_handler, update_merchant_network_address_handler,
    },
    network_handler::{
        handle_add_asset, handle_add_network, handle_get_all_networks, handle_get_network,
        handle_get_network_supported_assets,
    },
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
            .route("initdb", web::post().to(handle_init_bd))
            .route("/addnetwork", web::post().to(handle_add_network))
            .route(
                "/getassets",
                web::get().to(handle_get_network_supported_assets),
            )
            .route("/addassets", web::post().to(handle_add_asset))
            .route("/getnetwork", web::get().to(handle_get_network))
            .route("/getallnetworks", web::get().to(handle_get_all_networks))
            .route(
                "/createmerchantaccount",
                web::post().to(create_merchant_account_handler),
            )
            .route(
                "/addmerchantasset",
                web::post().to(add_merchant_asset_handler),
            )
            .route(
                "/removemerchantasset",
                web::post().to(remove_merchant_asset_handler),
            )
            .route(
                "/addmerchantnetwork",
                web::post().to(add_merchant_network_handler),
            )
            .route(
                "/updateaddress",
                web::post().to(update_merchant_network_address_handler),
            )
    })
    .bind(address)
    .unwrap()
    .run()
    .await
}
