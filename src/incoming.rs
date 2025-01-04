use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use lapin::Channel;
use serde::Deserialize;

use crate::{
    error_handling::{empty_string_as_none, CustomResponse},
    models::params::GeneralParams,
    publisher::publish_messages,
    responses::{ErrorResponse, Response},
};

pub async fn incoming(
    State(_publisher_channel): State<Channel>,
    Json(_payload): Json<GeneralParams>,
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
