use std::collections::HashMap;
use typed_builder::TypedBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct User{
    #[serde(rename = "user")]
    pub username: String,
    #[serde(rename = "passwd")]
    password: Option<String>,
    pub active: bool,
    pub extra: Option<HashMap<String, Value>>
    // change_password: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse{
    error: bool,
    code: u16,
    pub(crate) result: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserResponse{
    error: bool,
    code: u16,
}