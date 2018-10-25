use std::collections::HashMap;

use log::{debug, info};
use pretty_assertions::{assert_eq, assert_ne};
use reqwest::Url;

use super::Connection;

const URL: &str = "http://localhost:8529";

#[test]
fn test_setup() {
    env_logger::init();
}
#[test]
fn test_basic_auth() {
    // let _ = pretty_env_logger::try_init();
    let conn = Connection::establish_basic_auth(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let session = conn.get_session();
    let headers = session.get(URL).build().unwrap().headers();
    // let basic = headers.get::<Basic>().unwrap();
}

#[test]
fn test_jwt_auth() {
    // let _ = pretty_env_logger::try_init();
    let conn: Connection = Default::default();
    let jwt = conn.jwt_login("root", "KWNngteTps7XjrNv").unwrap();
    info!("JWT login success. Token: {}", jwt);
    // assert_eq!(false, true);
}
