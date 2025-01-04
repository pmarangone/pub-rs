use anyhow::anyhow;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use lapin::Channel;
use serde::Deserialize;

use crate::{
    error_handling::{empty_string_as_none, AppError, CustomResponse},
    responses::Response,
};

#[derive(Debug, Deserialize)]
pub struct GeneralParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    name: Option<String>,
    surname: Option<String>,
    description: Option<String>,
    age: Option<f32>,
}

pub async fn incoming(
    State(mut publisher_channel): State<Channel>,
    Json(payload): Json<GeneralParams>,
) -> impl IntoResponse {
    CustomResponse {
        status: StatusCode::OK,
        body: Response::Default(0f32),
    }
}
