use log::trace;
use pretty_assertions::assert_eq;

use arangors::Connection;

pub mod common;

use common::{test_setup, get_arangodb_host, get_normal_user, get_normal_password};

#[test]
fn test_get_collection() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let conn = Connection::establish_jwt(&host, &user, &password).unwrap();
    let database = conn.db("test_db").unwrap();
    trace!("{:?}", database.accessible_collections());
    let coll = database.collection("test_collection");
    assert_eq!(coll.is_err(), false);
    let coll = database.collection("test_collection_non_exists");
    assert_eq!(coll.is_err(), true);
}

#[test]
fn test_create_collection() {
    test_setup();
    let host = get_arangodb_host();
    let user = get_normal_user();
    let password = get_normal_password();

    let collection_name = "test1234";

    let conn = Connection::establish_jwt(&host, &user, &password).unwrap();
    let mut database = conn.db("test_db").unwrap();

    let coll = database.drop_collection(collection_name);
    assert_eq!(coll.is_err(), true);
    let coll = database.create_collection(collection_name);
    assert_eq!(coll.is_err(), false);
    let coll = database.drop_collection(collection_name);
    assert_eq!(coll.is_err(), false);
}