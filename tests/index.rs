#![allow(unused_imports)]
#![allow(unused_parens)]

use log::trace;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use crate::common::{collection, connection};
use arangors::{
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    ClientError, Connection, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};
use arangors::index::Index;

pub mod common;
#[maybe_async::test(
any(feature = "reqwest_blocking"),
async(any(feature = "reqwest_async"), tokio::test),
async(any(feature = "surf_async"), async_std::test)
)]
async fn test_create_index() {
    test_setup();
    let collection_name = "test_collection";
    let conn = connection().await;

    let mut database = conn.db("test_db").await.unwrap();
    let index = Index::persistent(vec!["password".to_string()], true, false).name("Testing");
    database.create_index(collection_name, &index).await.unwrap();


}