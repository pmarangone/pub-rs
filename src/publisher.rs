use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Channel,
    Connection, ConnectionProperties, Consumer, Result,
};

/// Connect to RabbitMQ and return the connection.
pub async fn connect_to_rabbitmq(addr: &str) -> Result<Connection> {
    let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
    tracing::debug!("Connected to RabbitMQ");
    Ok(conn)
}

/// Publish messages to the given queue in a loop.
pub async fn publish_messages(channel: Channel, queue_name: &str) -> Result<()> {
    let payload = b"Hello world!";
    loop {
        let confirm = channel
            .basic_publish(
                "",
                queue_name,
                BasicPublishOptions::default(),
                payload, // Pass as a reference
                BasicProperties::default(),
            )
            .await?
            .await?;

        assert_eq!(confirm, Confirmation::NotRequested);
        tracing::debug!(
            "Message published: {}",
            std::str::from_utf8(payload).unwrap()
        );
    }
}
