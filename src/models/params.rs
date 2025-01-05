// use crate::error_handling::empty_string_as_none;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralParams {
    pub name: String,
    pub surname: String,
    pub description: String,
    pub age: f32,
}
