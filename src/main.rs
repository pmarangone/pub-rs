use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use axum::routing::post;
use axum::{body::Bytes, routing::get, Router};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Channel,
    Connection, ConnectionProperties, Consumer, Result,
};

mod error_handling;
mod incoming;
mod publisher;
mod responses;

use incoming::incoming;
use publisher::connect_to_rabbitmq;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_reqwest_response=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    let conn = connect_to_rabbitmq(&addr).await?;

    let publisher_channel = conn.create_channel().await?;

    let balances: Arc<Mutex<HashMap<String, f32>>> = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/incoming", get(incoming))
        .with_state(publisher_channel)
        .layer(TraceLayer::new_for_http().on_body_chunk(
            |chunk: &Bytes, _latency: Duration, _span: &Span| {
                tracing::debug!("streaming {} bytes", chunk.len());
            },
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
