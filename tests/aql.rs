#![allow(unused_imports)]
#![allow(unused_parens)]

use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

use arangors::{AqlQuery, Connection, Document};
use common::{connection, test_setup};

use crate::common::{get_arangodb_host, get_root_password, get_root_user};

pub mod common;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

#[maybe_async::test(
    feature = "blocking",
    async(not(feature = "blocking"), tokio::test),
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
    feature = "blocking",
    async(not(feature = "blocking"), tokio::test),
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
    feature = "blocking",
    async(not(feature = "blocking"), tokio::test),
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

#[maybe_async::test(
    feature = "blocking",
    async(not(feature = "blocking"), tokio::test),
)]
async fn test_aql_try_bind() {
    test_setup();
    let conn = connection().await;
    let db = conn.db("test_db").await.unwrap();

    let user = User {
        username: "test2".to_owned(),
        password: "test2_pwd".to_owned(),
    };
    let aql = AqlQuery::builder()
        .query(r#"FOR i in test_collection FILTER i.username==@user.username AND i.password==@user.password return i"#)
        .try_bind("user", user)
        .unwrap()
        .build();
    let result: Vec<Document<User>> = db.aql_query(aql).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].document.password, "test2_pwd");
}
