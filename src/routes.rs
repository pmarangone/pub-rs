use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use deadpool_postgres::Pool;
use lapin::Channel;
use tracing::{error, info};

use crate::{
    db::primary_op::query_all,
    error_handling::{empty_string_as_none, CustomResponse},
    models::params::User,
    publisher::publish_messages,
    responses::{ErrorResponse, Response},
    AppState,
};

pub async fn incoming(
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<User>,
) -> impl IntoResponse {
    match publish_messages(&state.publisher_channel, "hello", _payload).await {
        Result::Ok(_) => CustomResponse {
            status: StatusCode::OK,
            body: Response::Default(0f32),
        },
        Result::Err(_) => CustomResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: Response::Error(ErrorResponse {
                error_message: "Something went wrong".to_string(),
            }),
        },
    }
}

pub async fn get_data(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = state
        .pool
        .get()
        .await
        .expect("Failed to get client from pool");
    match query_all(client).await {
        Ok(users) => CustomResponse {
            status: StatusCode::OK,
            body: Response::Users(users),
        },
        Err(_) => {
            error!("err");
            CustomResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: Response::Error(ErrorResponse {
                    error_message: "Something went wrong".to_string(),
                }),
            }
        }
    }
}
