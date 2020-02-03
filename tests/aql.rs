use pretty_assertions::assert_eq;
use serde::Deserialize;

use arangors::{AqlQuery, Connection, Document};
pub mod common;
use common::test_setup;
use crate::common::{get_arangodb_host, get_normal_user, get_normal_password, get_root_user, get_root_password};

const URL: &str = "http://localhost:8529/";

#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

#[test]
fn test_aql_str() {
    test_setup();
    let host = get_arangodb_host();
    let root_user = get_root_user();
    let root_password = get_root_password();

    let conn = Connection::establish_jwt(&host, &root_user, &root_password).unwrap();
    let db = conn.db("test_db").unwrap();
    let result: Vec<Document<User>> = db
        .aql_str(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}

#[test]
fn test_aql() {
    test_setup();
    let host = get_arangodb_host();
    let root_user = get_root_user();
    let root_password = get_root_password();

    let conn = Connection::establish_jwt(&host, &root_user, &root_password).unwrap();
    let db = conn.db("test_db").unwrap();
    let aql = AqlQuery::new(r#"FOR i in test_collection FILTER i.username=="test2" return i"#);
    let result: Vec<Document<User>> = db.aql_query(aql).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}

#[test]
fn test_aql_bind_vars() {
    test_setup();
    let host = get_arangodb_host();
    let root_user = get_root_user();
    let root_password = get_root_password();

    let conn = Connection::establish_jwt(&host, &root_user, &root_password).unwrap();
    let db = conn.db("test_db").unwrap();
    let aql = AqlQuery::new(r#"FOR i in test_collection FILTER i.username==@username return i"#)
        .bind_var("username", "test2");
    let result: Vec<Document<User>> = db.aql_query(aql).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}
