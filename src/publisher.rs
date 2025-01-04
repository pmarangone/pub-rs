use axum::extract::rejection::InvalidFormContentType;
use lapin::types::parsing::ParserError;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::parsing::ParserErrors, BasicProperties,
    Channel, Connection, ConnectionProperties,
};

use lapin::Error as LapinError;
use serde_json::Error as SerdeJsonError;

use crate::models::params::GeneralParams;

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
pub async fn publish_messages(
    channel: Channel,
    queue_name: &str,
    payload: GeneralParams,
) -> Result<(), PublishError> {
    let payload_bytes = serde_json::to_vec(&payload)?;

    for _ in 0..20 {
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
                    "Message published: {}",
                    std::str::from_utf8(&payload_bytes).unwrap()
                );
            }
        }
    }

    Ok(())
}
