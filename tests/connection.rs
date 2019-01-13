use log::info;
use pretty_assertions::{assert_eq, assert_ne};
use serde_json::value::Value;

use arangors::{AqlQuery, Connection, Cursor, Database};

const URL: &str = "http://localhost:8529/";

#[test]
fn setup() {
    env_logger::init();
}

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
    let database = conn.db("test_db");
    assert_eq!(database.is_none(), false);
    let database = conn.db("test_db_non_exist");
    assert_eq!(database.is_none(), true);
}

#[test]
fn test_get_version() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let version = conn.fetch_arango_version().unwrap();
    assert_eq!(version.license, "community");
    assert_eq!(version.server, "arango");

    let re = regex::Regex::new(r"3\.\d\.\d+").unwrap();
    assert_eq!(re.is_match(&version.version), true);
}

#[test]
fn test_fetch_current_database() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let database_info = conn.fetch_current_database().unwrap();
    assert_eq!(database_info.name, "_system");
    assert_eq!(database_info.is_system, true);
}

#[test]
fn test_basic_auth() {
    let conn = Connection::establish_basic_auth(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let session = conn.get_session();
    let resp = session.get(URL).send().unwrap();
    let headers = resp.headers();
    assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
    // let basic = headers.get::<Basic>().unwrap();
}

#[test]
fn test_jwt() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let session = conn.get_session();
    let resp = session.get(URL).send().unwrap();
    let headers = resp.headers();
    assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
}
