use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct User {
    #[serde(rename = "user")]
    pub username: String,
    #[serde(rename = "passwd")]
    password: Option<String>,
    pub active: bool,
    pub extra: Option<HashMap<String, Value>>, // change_password: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    error: bool,
    code: u16,
    pub(crate) result: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    error: bool,
    code: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDatabasesGetResponse {
    error: bool,
    code: u16,
    pub result: Value, // can be two formats based on parameter "full"
}

pub enum UserAccessLevel {
    None,
    ReadWrite,
    ReadOnly,
}

pub(crate) fn access_level_enum_to_str(level: UserAccessLevel) -> String {
    match level {
        UserAccessLevel::None => "none".into(),
        UserAccessLevel::ReadWrite => "rw".into(),
        UserAccessLevel::ReadOnly => "ro".into(),
    }
}
