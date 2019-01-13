use pretty_assertions::{assert_eq, assert_ne};
use serde_json::value::Value;

use arangors::{AqlQuery, Connection, Cursor, Database};

const URL: &str = "http://localhost:8529/";
const NEW_DB_NAME: &str = "example";

#[test]
fn setup() {
    env_logger::init();
}

#[test]
fn test_create_and_drop_database() {
    let mut conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let result = conn.create_database(NEW_DB_NAME).unwrap();
    assert_eq!(result, true);
    let result = conn.drop_database(NEW_DB_NAME).unwrap();
    assert_eq!(result, true);
}
