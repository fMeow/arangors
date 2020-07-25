#![allow(unused_imports)]
#![allow(unused_parens)]

use pretty_assertions::assert_eq;
use serde::Deserialize;

use arangors::{AqlQuery, Connection, Document};
use common::{connection, test_setup};

use crate::common::{get_arangodb_host, get_root_password, get_root_user};

pub mod common;

#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_aql_str() {
    test_setup();
    let conn = connection().await;
    let db = conn.db("test_db").await.unwrap();
    let result: Vec<Document<User>> = db
        .aql_str(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
        .await
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_aql() {
    test_setup();
    let conn = connection().await;
    let db = conn.db("test_db").await.unwrap();
    let aql = AqlQuery::builder()
        .query(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
        .build();
    let result: Vec<Document<User>> = db.aql_query(aql).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_aql_bind_vars() {
    test_setup();
    let conn = connection().await;
    let db = conn.db("test_db").await.unwrap();
    let aql = AqlQuery::builder()
        .query(r#"FOR i in test_collection FILTER i.username==@username AND i.password==@password return i"#)
        .bind_var("username", "test2")
        .bind_var("password", "test2_pwd")
        .build();
    let result: Vec<Document<User>> = db.aql_query(aql).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}
