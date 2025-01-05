use serde::Serialize;

use crate::models::params::User;

#[derive(Serialize)]
pub struct OKResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct UsersModel {
    pub count: i32,
    pub users: Vec<User>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
    Default(f32),
    Error(ErrorResponse),
    Users(UsersModel),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}
