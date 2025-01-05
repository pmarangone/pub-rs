use serde::Serialize;

use crate::models::params::User;

#[derive(Serialize)]
pub struct OKResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
    Default(f32),
    Error(ErrorResponse),
    Users(Vec<User>),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}
