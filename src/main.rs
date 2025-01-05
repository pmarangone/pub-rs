use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Bytes,
    routing::{get, post},
    Router,
};
// use db::primary_op::create_connection;
use models::params::User;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use anyhow::Result;

mod db;
mod error_handling;
mod models;
mod publisher;
mod responses;
mod routes;

use publisher::connect_to_rabbitmq;
use routes::{get_data, incoming};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_reqwest_response=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let mut client = create_connection();
    // match client {
    //     Ok(mut client) => {

    //     }
    //     Err(err) => {
    //         error!("Application initialization failed: {}", err);
    //         std::process::exit(1);
    //     }
    // }

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    let conn = connect_to_rabbitmq(&addr).await?;

    let publisher_channel = conn.create_channel().await?;

    // let payload = User {
    //     name: "Patrick".to_string(),
    //     surname: "Patrick".to_string(),
    //     description: "Patrick".to_string(),
    //     age: 10i32,
    // };

    // let result = publisher::publish_messages(publisher_channel, "hello", payload).await;

    let app = Router::new()
        .route("/incoming", post(incoming))
        .route("/data", get(get_data))
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

    Ok(())
}
