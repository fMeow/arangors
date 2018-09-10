use pretty_assertions::{assert_eq, assert_ne};
use serde_json::value::Value;

use arangors::{AqlQuery, Connection, Cursor, Database};
const URL: &str = "http://localhost:8529/";

#[test]
fn test_list_databases() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let databases = conn.list_all_database().unwrap();

    let order = databases == vec!["test_db", "_system"];
    let reverse_order = databases == vec!["_system", "test_db"];
    assert_eq!(order | reverse_order, true)
}

#[test]
fn test_get_url() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let url = conn.get_url().clone().into_string();
    assert_eq!(url, URL)
}

#[test]
fn test_refresh() {
    let mut conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let conn = conn.refresh().unwrap();
}

#[test]
fn test_get_database() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let database = conn.get_database("test_db");
    assert_eq!(database.is_none(), false);
    let database = conn.get_database("test_db_non_exist");
    assert_eq!(database.is_none(), true);
}
