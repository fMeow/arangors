#![allow(unused_imports)]
#![allow(unused_parens)]

use arangors::client::ClientExt;
use log::{info, trace, warn};
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::collections::HashMap;

use crate::common::{get_root_user, root_connection};
use arangors::{
    connection::Permission,
    user::{User, UserAccessLevel},
    ArangoError, Connection,
};
use common::{
    connection, get_arangodb_host, get_normal_password, get_normal_user, test_root_and_normal,
    test_setup,
};

pub mod common;

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_users_non_root() {
    test_setup();
    let conn = connection().await;

    let database = conn.db("test_db").await.unwrap();
    let users = database.users().await;

    assert_eq!(users.is_err(), true); // Should return 403 Forbidden
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_users() {
    test_setup();
    let conn = root_connection().await;

    let database = conn.db("test_db").await.unwrap();
    let users = database.users().await;
    match users {
        Ok(users) => {
            assert_eq!(users.len(), 2);
        }
        Err(err) => {
            println!("error: {:?}", err);
            assert!(false, "Fail to get users: {:?}", err)
        }
    }
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_user_crud_operations() {
    test_setup();
    let conn = root_connection().await;
    let database = conn.db("test_db").await.unwrap();

    // Create the test user
    let create_user_res = database
        .create_user(
            User::builder()
                .username("creation_test_user".into())
                .password(Some("test_password_123".into()))
                .active(true)
                .extra(None)
                .build(),
        )
        .await;
    assert_eq!(create_user_res.is_ok(), true);

    // test if there are 3 users now
    let users = database.users().await;
    match users {
        Ok(users) => {
            assert_eq!(users.len(), 3);
            assert_eq!(
                users
                    .iter()
                    .any(|user| user.username == "creation_test_user"),
                true
            );
        }
        Err(err) => {
            println!("error: {:?}", err);
            assert!(false, "Fail to get users: {:?}", err)
        }
    }
    // Update user
    let mut extra = HashMap::<String, Value>::new();
    extra.insert(
        "test_key".into(),
        serde_json::from_str("[ \"test_value\" ]").unwrap(),
    );
    let update_res = database
        .update_user(
            "creation_test_user".into(),
            User::builder()
                .username("creation_test_user".into())
                .password(Some("test_password_1234".into()))
                .active(true)
                .extra(Some(extra))
                .build(),
        )
        .await;
    assert_eq!(update_res.is_ok(), true);

    // test if there are 3 users now
    let users = database.users().await;
    match users {
        Ok(users) => {
            // Still 3 users
            assert_eq!(users.len(), 3);
            // but now with updated username
            assert_eq!(
                users
                    .iter()
                    .any(|user| user.username == "creation_test_user"),
                true
            );
            assert_eq!(users.iter().any(|user| user.extra.is_some()), true);
        }
        Err(err) => {
            println!("error: {:?}", err);
            assert!(false, "Fail to get users: {:?}", err)
        }
    }

    // Cleanup: delete temporary user
    let delete_res = database.delete_user("creation_test_user".into()).await;
    assert_eq!(delete_res.is_ok(), true);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_get_user_databases() {
    test_setup();
    let conn = root_connection().await;
    let database = conn.db("test_db").await.unwrap();

    // simple response
    let resp = database.user_databases(get_normal_user(), false).await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);

    // full response
    let resp = database.user_databases(get_root_user(), true).await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);

    // access-level for test_db
    let resp = database
        .user_db_access_level(get_root_user(), "test_db".into())
        .await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_user_db_access_put() {
    test_setup();
    let conn = root_connection().await;
    let database = conn.db("test_db").await.unwrap();

    let resp = database
        .user_db_access_put(get_normal_user(), "test_db".into(), UserAccessLevel::None)
        .await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);

    let resp = database
        .user_db_access_put(
            get_normal_user(),
            "test_db".into(),
            UserAccessLevel::ReadWrite,
        )
        .await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_user_db_collection_access_get() {
    test_setup();
    let conn = root_connection().await;
    let database = conn.db("test_db").await.unwrap();

    let resp = database
        .user_db_collection_access(
            get_normal_user(),
            "test_db".into(),
            "test_collection".into(),
        )
        .await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_user_db_collection_access_put() {
    test_setup();
    let conn = root_connection().await;
    let database = conn.db("test_db").await.unwrap();

    let resp = database
        .user_db_collection_access_put(
            get_normal_user(),
            "test_db".into(),
            "test_collection".into(),
            UserAccessLevel::ReadOnly,
        )
        .await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);

    let resp = database
        .user_db_collection_access_put(
            get_normal_user(),
            "test_db".into(),
            "test_collection".into(),
            UserAccessLevel::ReadWrite,
        )
        .await;
    trace!("resp: {:?}", resp);
    assert_eq!(resp.is_ok(), true);
}
