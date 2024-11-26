use crate::{
    auth::jwt::{admin_jwt_validator, merchant_jwt_validator, pending_payment_jwt_validator},
    handlers::{
        admin_handlers::{
            admin_login_handler, create_admin_with_invite_handler, create_super_admin_handler,
            generate_admin_invite_handler, verify_otp_handler,
        },
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
        payment_handlers::{evm_create_payment_request_handler, validate_evm_payment_handler},
    },
};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
pub fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/initdb", web::post().to(handle_init_bd))
        .route(
            "/createsuperadmin",
            web::post().to(create_super_admin_handler),
        )
        .route(
            "/createadminwithinvite",
            web::post().to(create_admin_with_invite_handler),
        )
        .route(
            "/getassets",
            web::get().to(handle_get_network_supported_assets),
        )
        .route("/getnetwork", web::get().to(handle_get_network))
        .route("/getallnetworks", web::get().to(handle_get_all_networks));
}

pub fn configure_merchant_api_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(merchant_jwt_validator);

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

pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(admin_jwt_validator);

    cfg.service(
        web::scope("/admin")
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(admin_login_handler))
                    .route("/otp/verify", web::post().to(verify_otp_handler)),
            )
            .service(
                web::scope("")
                    .wrap(auth)
                    .route(
                        "/createadmininvite",
                        web::post().to(generate_admin_invite_handler),
                    )
                    .route("/addnetwork", web::post().to(handle_add_network))
                    .route("/addasset", web::post().to(handle_add_asset)),
            ),
    );
}

pub fn configure_payment_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(pending_payment_jwt_validator);

    cfg.service(
        web::scope("/user")
            .service(web::scope("/auth").route(
                "/make-payment",
                web::post().to(evm_create_payment_request_handler),
            ))
            .service(web::scope("").wrap(auth).route(
                "/verify-payment",
                web::post().to(validate_evm_payment_handler),
            )),
    );
}
