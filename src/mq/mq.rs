use crate::{
    core::{evm::evm::verify_signed_transaction, sol::sol::verify_sol_signed_transaction},
    types::types::{TransactionVerificationMessage, WebhookPayload},
    utils::utils::send_webhook_notification,
};
use chrono::Utc;
use futures::StreamExt;
use lapin::{
    message::Delivery, options::*, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Consumer,
};
use sqlx::PgPool;
use tracing::error as TracingError;

pub async fn start_consumer(
    rabbitmq_url: &str,
    queue_name: &str,
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::connect(rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = connection.create_channel().await?;

    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    tracing::info!("Listening on queue: {}", queue_name);

    let consumer: Consumer = channel
        .basic_consume(
            queue_name,
            "consumer_tag",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = consumer;

    tracing::info!("Waiting for messsages...");
    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let db_pool = pool.clone();
            tokio::spawn(async move {
                if let Err(err) = handle_message(delivery, &db_pool).await {
                    eprintln!("Error handling message: {:?}", err);
                }
            });
        }
    }

    Ok(())
}

async fn handle_message(
    delivery: Delivery,
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = String::from_utf8(delivery.data.clone()).unwrap_or_default();
    tracing::info!("Received message: {}", message);

    let message: TransactionVerificationMessage = match serde_json::from_slice(&delivery.data) {
        Ok(msg) => msg,
        Err(e) => {
            TracingError!(error = ?e, "Failed to deserialize message");
            eprintln!("Failed to deserialize message: {}", e);
            delivery.nack(BasicNackOptions::default()).await?;
            return Ok(());
        }
    };
    tracing::info!("Received message: {:?}", message);

    let verification_result = if message.network.to_lowercase().contains("sol") {
        verify_sol_signed_transaction(
            &pool,
            message.pending_payment_id,
            &message.rpc_url,
            &message.tx_hash,
        )
        .await
    } else {
        verify_signed_transaction(
            &pool,
            message.pending_payment_id,
            &message.rpc_url,
            &message.tx_hash,
        )
        .await
    };

    match verification_result {
        Ok((id, webhook_url)) => {
            tracing::info!("Transaction verified successfully!");
            delivery.ack(BasicAckOptions::default()).await?;
            tracing::info!("Payment creation Successful: Payment id {}", id);

            let payload = WebhookPayload {
                payment_id: id,
                status: "completed".to_string(),
                tx_hash: message.tx_hash.clone(),
                timestamp: Utc::now().to_rfc3339(),
            };

            let payload_json = serde_json::to_string(&payload)?;

            send_webhook_notification(&webhook_url, &payload_json).await?;
        }
        Err(e) => {
            eprintln!("Verification failed: {}", e);
            delivery.nack(BasicNackOptions::default()).await?;
            TracingError!(error = ?e, "Payment creation error");
        }
    }

    delivery.ack(BasicAckOptions::default()).await?;
    Ok(())
}

pub async fn publish_message(
    rabbitmq_url: &str,
    queue_name: &str,
    message: TransactionVerificationMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::connect(rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = connection.create_channel().await?;

    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let payload = serde_json::to_vec(&message)?;

    channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?;

    tracing::info!("Message sent: {:?}", message);
    Ok(())
}
