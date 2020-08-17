//! Sync and async version share the exactly same API, except that async API
//! must be awaited.

#![allow(unused_imports)]
#![allow(unused_parens)]

use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use arangors::{AqlQuery, Connection};
use std::collections::HashMap;

const URL: &str = "http://localhost:8529";

#[derive(Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
}

#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
#[cfg_attr(feature = "reqwest_blocking", maybe_async::must_be_sync)]
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
    // use bind_var for any struct that can be converted into serde_json::Value
    let json = serde_json::to_value(&user).unwrap();
    let aql = AqlQuery::builder()
        .query("INSERT @user INTO @@collection LET result = NEW RETURN result")
        .bind_var("@collection", collection)
        .bind_var("user", json)
        .build();

    let result: Vec<User> = database.aql_query(aql).await.unwrap();
    println!("{:?}", result);

    let jane_doe = User {
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        email: "jane.done@who.com".to_string(),
    };
    // use try_bind for any serializable struct
    let aql = AqlQuery::builder()
        .query("INSERT @user INTO @@collection LET result = NEW RETURN result")
        .bind_var("@collection", collection)
        .try_bind("user", jane_doe)
        .unwrap()
        .build();

    let result: Vec<User> = database.aql_query(aql).await.unwrap();
    println!("{:?}", result);

    let homer_simpson = User {
        first_name: "Homer".to_string(),
        last_name: "Simpson".to_string(),
        email: "homer.sompson@springfield.com".to_string(),
    };

    let mut map: HashMap<&str, Value> = HashMap::new();
    map.insert("@collection", Value::from(collection));
    map.insert("user", serde_json::to_value(homer_simpson).unwrap());

    // use bind_vars to pass a HashMap of bind variables
    let aql = AqlQuery::builder()
        .query("INSERT @user INTO @@collection LET result = NEW RETURN result")
        .bind_vars(map)
        .build();

    let result: Vec<User> = database.aql_query(aql).await.unwrap();
    println!("{:?}", result);
}

#[cfg(not(any(
    feature = "reqwest_blocking",
    feature = "reqwest_async",
    feature = "surf_async"
)))]
fn main() {}
