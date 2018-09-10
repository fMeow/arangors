use pretty_assertions::{assert_eq, assert_ne};
use serde_json::value::Value;

use arangors::{AqlQuery, Connection, Cursor, Database};
const URL: &str = "http://localhost:8529";

#[test]
fn test_list_databases() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let databases = conn.list_all_database().unwrap();

    let order = databases == vec!["test_db", "_system"];
    let reverse_order = databases == vec!["_system", "test_db"];
    assert_eq!(order | reverse_order, true)
}
