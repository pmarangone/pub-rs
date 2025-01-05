// use crate::error_handling::empty_string_as_none;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub surname: String,
    pub description: String,
    pub age: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionModel {
    id: u64,
    amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub to: String,
}
