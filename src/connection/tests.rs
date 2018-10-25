use std::collections::HashMap;

use log::{debug, info};
use pretty_assertions::{assert_eq, assert_ne};
use reqwest::{header::AUTHORIZATION, Url};

use super::Connection;

const URL: &str = "http://localhost:8529";
const USERNAME: &str = "root";
const PASSWORD: &str = "KWNngteTps7XjrNv";

#[test]
fn test_setup() {
    env_logger::init();
}

#[test]
fn test_jwt_auth() {
    // let _ = pretty_env_logger::try_init();
    let conn: Connection = Default::default();
    let jwt = conn.jwt_login("root", "KWNngteTps7XjrNv").unwrap();
    info!("JWT login success. Token: {}", jwt);
    let not_empty = jwt.len() > 1;
    assert_eq!(not_empty, true);
}
