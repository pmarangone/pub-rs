use crate::error_handling::empty_string_as_none;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    name: Option<String>,
    surname: Option<String>,
    description: Option<String>,
    age: Option<f32>,
}
