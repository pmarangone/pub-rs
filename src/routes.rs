use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use lapin::Channel;
use serde::Deserialize;
use tracing::{error, info};

use crate::{
    db::primary_op::query_all,
    error_handling::{empty_string_as_none, CustomResponse},
    models::params::User,
    publisher::publish_messages,
    responses::{ErrorResponse, Response},
};

pub async fn incoming(
    State(_publisher_channel): State<Channel>,
    Json(_payload): Json<User>,
) -> impl IntoResponse {
    match publish_messages(_publisher_channel, "hello", _payload).await {
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

pub async fn get_data() -> impl IntoResponse {
    match query_all().await {
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
