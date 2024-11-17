use crate::{
    auth::auth::jwt_validator,
    handlers::{
        handle_init_bd,
        merchant_handlers::{
            add_merchant_asset_handler, add_merchant_network_handler,
            create_merchant_account_handler, merchant_login_handler, remove_merchant_asset_handler,
            update_merchant_network_address_handler,
        },
        network_handler::{
            handle_add_asset, handle_add_network, handle_get_all_networks, handle_get_network,
            handle_get_network_supported_assets,
        },
    },
};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
pub fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/initdb", web::post().to(handle_init_bd))
        .route("/addnetwork", web::post().to(handle_add_network))
        .route(
            "/getassets",
            web::get().to(handle_get_network_supported_assets),
        )
        .route("/addassets", web::post().to(handle_add_asset))
        .route("/getnetwork", web::get().to(handle_get_network))
        .route("/getallnetworks", web::get().to(handle_get_all_networks));
}

pub fn configure_merchant_api_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_validator);

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/merchantlogin", web::post().to(merchant_login_handler))
                    .route(
                        "/merchantregister",
                        web::post().to(create_merchant_account_handler),
                    ),
            )
            .service(
                web::scope("/merchant")
                    .wrap(auth)
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
                    ),
            ),
    );
}
