use pretty_assertions::{assert_eq, assert_ne};
use serde_derive::{Deserialize, Serialize};

use arangors::Connection;
const URL: &str = "http://localhost:8529/";

#[test]
fn setup() {
    let _ = pretty_env_logger::try_init();
}

#[test]
fn test_has_collection() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let database = conn.db("test_db").unwrap();
    println!("{:?}", database.list_user_collections());
    let coll = database.has_collection("test_collection");
    assert_eq!(coll, true);
    let coll = database.has_collection("test_collection_non_exists");
    assert_eq!(coll, false);
    let coll = database.has_system_collection("_apps");
    assert_eq!(coll, true);
    let coll = database.has_system_collection("none");
    assert_eq!(coll, false);
}

#[test]
fn test_get_collection() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let database = conn.db("test_db").unwrap();
    println!("{:?}", database.list_user_collections());
    let coll = database.get_collection("test_collection");
    assert_eq!(coll.is_none(), false);
    let coll = database.get_collection("test_collection_non_exists");
    assert_eq!(coll.is_none(), true);
}
