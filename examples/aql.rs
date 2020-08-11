//! Sync and async version share the exactly same API, except that async API
//! must be awaited.

#![allow(unused_imports)]
#![allow(unused_parens)]

use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use arangors::{AqlQuery, Connection};

const URL: &str = "http://localhost:8529";

#[derive(Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
}

// See this example when you want an async code
#[cfg(any(feature = "reqwest_async", feature = "surf_async"))]
#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
async fn main() {
    env_logger::init();

    let conn = Connection::establish_jwt(URL, "username", "password")
        .await
        .unwrap();

    let database = conn.db("test_db").await.unwrap();
    let aql = AqlQuery::builder()
        .query("FOR u IN test_collection LIMIT 3 RETURN u")
        .build();
    println!("{:?}", aql);
    println!("{:?}", serde_json::to_string(&aql).unwrap());

    let resp: Vec<Value> = database.aql_query(aql).await.unwrap();
    println!("{:?}", resp);

    let collection = "test_collection";
    let user = User {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@who.com".to_string(),
    };
    let json = serde_json::to_value(&user).unwrap();
    let aql = AqlQuery::builder()
        .query("INSERT @user INTO @@collection LET result = NEW RETURN result")
        .bind_var("@collection", collection)
        .bind_var("user", json)
        .build();

    let result: Vec<User> = database.aql_query(aql).await.unwrap();
    println!("{:?}", result)
}

// See this example when you want an blocking code
#[cfg(feature = "reqwest_blocking")]
fn main() {
    env_logger::init();

    let conn = Connection::establish_jwt(URL, "username", "password").unwrap();

    let database = conn.db("test_db").unwrap();
    let aql = AqlQuery::builder()
        .query("FOR u IN test_collection LIMIT 3 RETURN u")
        .build();
    println!("{:?}", aql);
    println!("{:?}", serde_json::to_string(&aql).unwrap());

    let resp: Vec<Value> = database.aql_query(aql).unwrap();
    println!("{:?}", resp);
}

#[cfg(not(any(
    feature = "reqwest_blocking",
    feature = "reqwest_async",
    feature = "surf_async"
)))]
fn main() {}
