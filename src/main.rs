mod admin;
mod auth;
mod core;
mod db;
mod error;
mod handlers;
mod merchant;
mod mq;
mod network;
mod routes;
mod types;
mod utils;

use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use db::db_init::connect_db;
use dotenv::dotenv;
use env_logger::Env;
use mq::mq::start_consumer;
use routes::routes::{
    configure_admin_routes, configure_merchant_api_routes, configure_payment_routes,
    configure_public_routes,
};
use std::collections::HashMap;
use tokio::spawn;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let rabbitmq_url = std::env::var("RABBITMQ_URL").expect("RABBITMQ_URL not set");
    let queue_name = std::env::var("QUEUE_NAME").expect("QUEUE_NAME not set");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    let address: &str = "0.0.0.0:8080";
    info!("Starting micrors at http://{}", address);

    let pool = connect_db().await.expect("error conneting to db");

    let consumer_pool = pool.clone();
    spawn(async move {
        if let Err(err) = start_consumer(&rabbitmq_url, &queue_name, &consumer_pool).await {
            tracing::error!("Error running RabbitMQ consumer: {:?}", err);
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .app_data(web::Data::new(pool.clone())) //uses Arc
            .configure(configure_public_routes)
            .configure(configure_merchant_api_routes)
            .configure(configure_admin_routes)
            .configure(configure_payment_routes)
    })
    .bind(address)
    .unwrap()
    .run()
    .await
}
