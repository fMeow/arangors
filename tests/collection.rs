use log::trace;
use pretty_assertions::assert_eq;

use arangors::Connection;

const URL: &str = "http://localhost:8529/";

pub mod common;

use common::{test_setup, NORMAL_PASSWORD, NORMAL_USERNAME};

#[test]
fn test_get_collection() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD).unwrap();
    let database = conn.db("test_db").unwrap();
    trace!("{:?}", database.accessible_collections());
    let coll = database.collection("test_collection");
    assert_eq!(coll.is_err(), false);
    let coll = database.collection("test_collection_non_exists");
    assert_eq!(coll.is_err(), true);
}
