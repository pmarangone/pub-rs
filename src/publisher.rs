use apalis::prelude::*;
use apalis_amqp::AmqpBackend;
use apalis_redis::{Config, RedisStorage};
use lapin::{
    options::*, publisher_confirm::Confirmation, types::parsing::ParserErrors, BasicProperties,
    Channel, Connection, ConnectionProperties,
};

use lapin::Error as LapinError;
use serde_json::Error as SerdeJsonError;
use tracing::{error, info};

use crate::models::params::{Email, TransactionModel, User};

pub async fn connect_to_rabbitmq(addr: &str) -> lapin::Result<Connection> {
    let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
    tracing::debug!("Connected to RabbitMQ");
    Ok(conn)
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PublishError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] SerdeJsonError),
    #[error("Lapin error: {0}")]
    Lapin(#[from] LapinError),
}

/// Publish messages to the given queue in a loop.
/// # TODO: Refactor to receive a transactionmodel
pub async fn publish_messages(
    channel: &Channel,
    queue_name: &str,
    payload: TransactionModel,
) -> Result<(), PublishError> {
    let payload_bytes = serde_json::to_vec(&payload)?;

    for _ in 0..2 {
        let confirm = channel
            .basic_publish(
                "",
                queue_name,
                BasicPublishOptions::default(),
                &payload_bytes, // Pass as a reference
                BasicProperties::default(),
            )
            .await?
            .await?;

        match confirm {
            Ack => {
                tracing::debug!(
                    "Message published: {}",
                    std::str::from_utf8(&payload_bytes).unwrap()
                );
            }
            Nack => {
                tracing::debug!(
                    "Message wasn't published: {}",
                    std::str::from_utf8(&payload_bytes).unwrap()
                );
            }
        }
    }

    Ok(())
}

pub async fn publish_task(
    channel: &Channel,
    transaction: &TransactionModel,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = serde_json::to_vec(transaction)?;
    channel
        .basic_publish(
            "",
            "transaction_queue",
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?;

    println!("Task published: {:?}", transaction);
    Ok(())
}

//This can be in another part of the program or another application eg a http server
pub async fn produce_route_jobs() -> Result<(), anyhow::Error> {
    info!("Called producer!");

    let redis_url = "redis://localhost:6379";
    let conn = apalis_redis::connect(redis_url)
        .await
        .expect("Could not connect");
    let config_storage = Config::default();
    let conf = config_storage.set_namespace("email-worker");
    let mut storage = RedisStorage::new_with_config(conn, conf);

    match storage.push(TransactionModel { id: 1, amount: 10 }).await {
        Ok(_) => {
            info!("Job request sent successfully.");
        }
        Err(err) => {
            error!("Job request failed. {:?}", err);
        }
    }

    Ok(())
}
//This can be in another part of the program or another application eg a http server
pub async fn amqp_produce_route_jobs() -> Result<(), anyhow::Error> {
    info!("Called producer!");

    let amqp_addr = "amqp://127.0.0.1:5672/%2f";

    let conn = connect_to_rabbitmq(&amqp_addr).await?;
    let amqp_channel = conn.create_channel().await?;

    let amqp_queue = amqp_channel
        .queue_declare(
            "transaction_queue",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    let config_storage = apalis_amqp::utils::Config::new("transaction_queue");
    let mut backend: AmqpBackend<TransactionModel> =
        AmqpBackend::new_with_config(amqp_channel, amqp_queue, config_storage);

    match backend
        .enqueue(TransactionModel { id: 1, amount: 42 })
        .await
    {
        Ok(_) => {
            info!("Job request sent successfully.");
        }
        Err(err) => {
            error!("Job request failed. {:?}", err);
        }
    }

    Ok(())
}
