use log::trace;
use pretty_assertions::assert_eq;

use arangors::Connection;
use common::{
    test_root_and_normal, test_setup, get_arangodb_host, get_root_user, get_root_password, get_normal_user, get_normal_password,
};

pub mod common;

const NEW_DB_NAME: &str = "example";

#[test]
fn test_create_and_drop_database() {
    test_setup();
    let host = get_arangodb_host();
    let root_user = get_root_user();
    let root_password = get_root_password();

    let conn = Connection::establish_jwt(&host, &root_user, &root_password)
        .unwrap()
        .into_admin()
        .unwrap();

    let result = conn.create_database(NEW_DB_NAME);
    if let Err(e) = result {
        assert!(false, "Fail to create database: {:?}", e)
    };
    let result = conn.db(NEW_DB_NAME);
    assert_eq!(result.is_err(), false);

    let mut conn = conn;
    let result = conn.drop_database(NEW_DB_NAME);
    if let Err(e) = result {
        assert!(false, "Fail to drop database: {:?}", e)
    };
    let result = conn.db(NEW_DB_NAME);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_fetch_current_database_info() {
    test_setup();

    fn fetch_current_database(user: &str, passwd: &str) {
        let host = get_arangodb_host();
        let conn = Connection::establish_jwt(&host, user, passwd).unwrap();
        let db = conn.db("test_db").unwrap();
        let info = db.info();
        match info {
            Ok(info) => {
                trace!("{:?}", info);
                assert_eq!(info.is_system, false)
            }
            Err(e) => assert!(false, "Fail to drop database: {:?}", e),
        }
    }
    test_root_and_normal(fetch_current_database);
}

#[test]
fn test_get_version() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password).unwrap();
    let db = conn.db("test_db").unwrap();
    let version = db.arango_version().unwrap();
    trace!("{:?}", version);
    assert_eq!(version.license, "community");
    assert_eq!(version.server, "arango");

    let re = regex::Regex::new(r"3\.\d\.\d+").unwrap();
    assert_eq!(re.is_match(&version.version), true);
}
