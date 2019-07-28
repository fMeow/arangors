use pretty_assertions::assert_eq;

use arangors::connection::Permission;
use arangors::Connection;
use common::{test_root_and_normal, test_setup, NORMAL_PASSWORD, NORMAL_USERNAME};

pub mod common;

const URL: &str = "http://localhost:8529/";

#[test]
fn test_list_databases() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD).unwrap();
    let dbs = conn.accessible_databases().unwrap();

    assert_eq!(dbs.contains_key("test_db"), true);
    let db_permission = dbs.get("test_db").unwrap();
    match db_permission {
        Permission::ReadOnly | Permission::NoAccess => {
            assert!(false, "Invalid permission {:?}", db_permission)
        }
        _ => {}
    };
}

#[test]
fn test_get_url() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD).unwrap();
    let url = conn.get_url().clone().into_string();
    assert_eq!(url, URL)
}

#[test]
fn test_get_database() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD).unwrap();
    let database = conn.db("test_db");
    assert_eq!(database.is_err(), false);
    let database = conn.db("test_db_non_exist");
    assert_eq!(database.is_err(), true);
}

#[test]
fn test_basic_auth() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD).unwrap();
    let session = conn.get_session();
    let resp = session.get(URL).send().unwrap();
    let headers = resp.headers();
    assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
}

#[test]
fn test_jwt() {
    test_setup();
    fn jwt(user: &str, passwd: &str) {
        let conn = Connection::establish_jwt(URL, user, passwd).unwrap();
        let session = conn.get_session();
        let resp = session.get(URL).send().unwrap();
        let headers = resp.headers();
        assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
    }
    test_root_and_normal(jwt);
}
