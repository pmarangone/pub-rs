use serde::Serialize;

#[derive(Serialize)]
pub struct OKResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
    Default(f32),
    Error(ErrorResponse),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}
