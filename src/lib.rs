//! # arangors
//!
//! [![Build Status](https://github.com/fMeow/arangors/workflows/CI%20%28Linux%29/badge.svg?branch=master)](https://github.com/fMeow/arangors/actions)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/arangors.svg)](https://crates.io/crates/arangors)
//! [![arangors](https://docs.rs/arangors/badge.svg)](https://docs.rs/arangors)
//!
//! `arangors` is an intuitive rust client for [ArangoDB](https://www.arangodb.com/),
//! inspired by [pyArango](https://github.com/tariqdaouda/pyArango).
//!
//! `arangors` enables you to connect with ArangoDB server, access to database,
//! execute AQL query, manage ArangoDB in an easy and intuitive way,
//! both `async` and plain synchronous code with any HTTP ecosystem you love.
//!
//! ## Philosophy of arangors
//!
//! `arangors` is targeted at ergonomic, intuitive and OOP-like API for
//! ArangoDB, both top level and low level API for users' choice.
//!
//! Overall architecture of ArangoDB:
//!
//! > databases -> collections -> documents/edges
//!
//! In fact, the design of `arangors` just mimic this architecture, with a
//! slight difference that in the top level, there is a connection object on top
//! of databases, containing a HTTP client with authentication information in
//! HTTP headers.
//!
//! Hierarchy of arangors:
//! > connection -> databases -> collections -> documents/edges
//!
//! ## Features
//!
//! By now, the available features of arangors are:
//!
//! - make connection to ArangoDB
//! - get list of databases and collections
//! - fetch database and collection info
//! - create and delete database or collections
//! - full featured AQL query
//! - support both `async` and sync
//!
//! ## TODO
//!
//! - (Done) Milestone 0.1.x
//!
//!     Synchronous connection based on `reqwest` and full featured AQL query.
//!
//! - (X) Milestone 0.2.x
//!
//!     Fill the unimplemented API in `Connection`, `Database`, `Collection` and
//!     `Document`.
//!
//!     ~~In this stage, all operations available for database, collection and
//!     document should be implemented.~~
//!
//!     Well, I am too lazy to fill all API, as the AQL syntax suffices in most
//!     cases. Maybe fulfill this goal in 0.4.x .
//!
//! - (Done) Milestone 0.3.x
//!
//!     Implement both sync and async client. Also, offers a way to use custom
//!     HTTP client ecosystem.
//!
//! - (WIP) Milestone 1.0.x
//!
//!     Provides the API related to:
//!     - (X) Graph Management
//!     - (X) Index Management
//!     - ( ) User Management
//!
//!     In this stage, all operations available for database, collection and
//!     document should be implemented.
//!
//! ## Glance
//!
//! ### Use Different HTTP Ecosystem, Regardless of Async or Sync
//!
//! You can switch to different HTTP ecosystem with a feature gate, or implement
//! the Client yourself (see examples).
//!
//! Currently out-of-box supported ecosystem are:
//! - `reqwest_async`
//! - `reqwest_blocking`
//! - `surf_async`
//!
//! By default, `arangors` use `reqwest_async` as underling HTTP Client to
//! connect with ArangoDB. You can switch other ecosystem in feature gate:
//!
//! ```toml
//! [dependencies]
//! arangors = { version = "0.4", features = ["surf_async"], default-features = false }
//! ```
//!
//! Or if you want to stick with other ecosystem that are not listed in the
//! feature gate, you can get vanilla `arangors` without any HTTP client
//! dependency:
//!
//! ```toml
//! [dependencies]
//! ## This one is async
//! arangors = { version = "0.4", default-features = false }
//! ## This one is synchronous
//! arangors = { version = "0.4", features = ["blocking"], default-features = false }
//! ```
//!
//! Thanks to `maybe_async`, `arangors` can unify sync and async API and toggle
//! with a feature gate. Arangors adopts async first policy.
//!
//! ### Connection
//!
//! There is three way to establish connections:
//! - jwt
//! - basic auth
//! - no authentication
//!
//! So are the `arangors` API.
//!
//! Example:
//!
//! - With authentication
//!
//! ```rust
//! use arangors::Connection;
//!
//! # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! // (Recommended) Handy functions
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! let conn = Connection::establish_basic_auth("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! - Without authentication, only use in evaluation setting
//!
//! ``` rust, ignore
//! # use arangors::Connection;
//! let conn = Connection::establish_without_auth("http://localhost:8529").await.unwrap();
//! ```
//!
//! ### Database && Collection
//!
//! ```rust
//! use arangors::Connection;
//!
//! # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! # let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//! #     .await
//! #     .unwrap();
//! let db = conn.db("test_db").await.unwrap();
//! let collection = db.collection("test_collection").await.unwrap();
//! # }
//! ```
//!
//! ### AQL Query
//!
//! All [AQL](https://www.arangodb.com/docs/stable/aql/index.html) query related functions are associated with database, as AQL query
//! is performed at database level.
//!
//! There are several way to execute AQL query, and can be categorized into two
//! classes:
//!
//! - batch query with cursor
//!     - `aql_query_batch`
//!     - `aql_next_batch`
//!
//! - query to fetch all results
//!     - `aql_str`
//!     - `aql_bind_vars`
//!     - `aql_query`
//!
//! This later ones provide a convenient high level API, whereas batch
//! queries offer more control.
//!
//! #### Typed or Not Typed
//!
//! Note that results from ArangoDB server, e.x. fetched documents, can be
//! strong typed given deserializable struct, or arbitrary JSON object with
//! `serde::Value`.
//!
//! ```rust
//! # use arangors::Connection;
//! # use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! struct User {
//!     pub username: String,
//!     pub password: String,
//! }
//!
//! # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! # let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//! #    .await
//! #    .unwrap();
//! # let db = conn.db("test_db").await.unwrap();
//! // Typed
//! let resp: Vec<User> = db
//!     .aql_str("FOR u IN test_collection RETURN u")
//!     .await
//!     .unwrap();
//! // Not typed: Arbitrary JSON objects
//! let resp: Vec<serde_json::Value> = db
//!     .aql_str("FOR u IN test_collection RETURN u")
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! #### Batch query
//!
//! `arangors` offers a way to manually handle batch query.
//!
//! Use `aql_query_batch` to get a cursor, and use `aql_next_batch` to fetch
//! next batch and update cursor with the cursor.
//!
//! ```rust
//! # use arangors::{ClientError,Connection, AqlQuery};
//!
//! # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! # let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//! #     .await
//! #     .unwrap();
//! # let db = conn.db("test_db").await.unwrap();
//!
//! let aql = AqlQuery::builder()
//!     .query("FOR u IN @@collection LIMIT 3 RETURN u")
//!     .bind_var("@collection", "test_collection")
//!     .batch_size(1)
//!     .count(true)
//!     .build();
//!
//! // fetch the first cursor
//! let mut cursor = db.aql_query_batch(aql).await.unwrap();
//! // see metadata in cursor
//! println!("count: {:?}", cursor.count);
//! println!("cached: {}", cursor.cached);
//! let mut results: Vec<serde_json::Value> = Vec::new();
//! loop {
//!     if cursor.more {
//!         let id = cursor.id.unwrap().clone();
//!         // save data
//!         results.extend(cursor.result.into_iter());
//!         // update cursor
//!         cursor = db.aql_next_batch(id.as_str()).await.unwrap();
//!     } else {
//!         break;
//!     }
//! }
//! println!("{:?}", results);
//! # }
//! ```
//!
//! #### Fetch All Results
//!
//! There are three functions for AQL query that fetch all results from
//! ArangoDB. These functions internally fetch batch results one after another
//! to get all results.
//!
//! The functions for fetching all results are listed as bellow:
//!
//! ##### `aql_str`
//!
//! This function only accept a AQL query string.
//!
//! Here is an example of strong typed query result with `aql_str`:
//!
//! ```rust
//! # use arangors::Connection;
//! # use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! struct User {
//!     pub username: String,
//!     pub password: String,
//! }
//!
//! # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! # let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//! #     .await
//! #     .unwrap();
//! # let db = conn.db("test_db").await.unwrap();
//! let result: Vec<User> = db
//!     .aql_str(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! ##### `aql_bind_vars`
//!
//! This function can be used to start a AQL query with bind variables.
//!
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! # use std::collections::HashMap;
//! use arangors::{Connection, Document};
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct User {
//!     pub username: String,
//!     pub password: String,
//! }
//!
//! # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! # let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//! #     .await
//! #     .unwrap();
//! # let db = conn.db("test_db").await.unwrap();
//!
//! let mut vars = HashMap::new();
//! let user = User {
//!     username: "test".to_string(),
//!     password: "test_pwd".to_string(),
//! };
//! vars.insert("user", serde_json::value::to_value(&user).unwrap());
//! let result: Vec<Document<User>> = db
//!     .aql_bind_vars(r#"FOR i in test_collection FILTER i==@user return i"#, vars)
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! ##### `aql_query`
//!
//! This function offers all the options available to tweak a AQL query.
//! Users have to construct a `AqlQuery` object first. And `AqlQuery` offer all
//! the options needed to tweak AQL query. You can set batch size, add bind
//! vars, limit memory, and all others
//! options available.
//!
//! ```rust
//! use arangors::{AqlQuery, Connection, Cursor, Database};
//! use serde_json::value::Value;
//!
//! # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! # let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//! #     .await
//! #     .unwrap();
//! # let db = conn.db("test_db").await.unwrap();
//!
//! let aql = AqlQuery::builder()
//!     .query("FOR u IN @@collection LIMIT 3 RETURN u")
//!     .bind_var("@collection", "test_collection")
//!     .batch_size(1)
//!     .count(true)
//!     .build();
//!
//! let resp: Vec<Value> = db.aql_query(aql).await.unwrap();
//! println!("{:?}", resp);
//! # }
//! ```
//!
//! ### Contributing
//!
//! Contributions and feed back are welcome following Github workflow.
//!
//! ### License
//!
//! `arangors` is provided under the MIT license. See [LICENSE](./LICENSE).
//! An ergonomic [ArangoDB](https://www.arangodb.com/) client for rust.
#![allow(unused_parens)]

#[cfg(any(
    feature = "reqwest_async",
    feature = "reqwest_blocking",
    feature = "surf_async"
))]
pub use crate::connection::Connection;
pub use crate::{
    aql::{AqlOptions, AqlQuery, Cursor},
    collection::Collection,
    connection::GenericConnection,
    database::Database,
    document::Document,
    error::{ArangoError, ClientError},
};

pub mod analyzer;
pub mod aql;
pub mod client;
pub mod collection;
pub mod connection;
pub mod database;
pub mod document;
pub mod error;
pub mod graph;
pub mod index;
mod query;
mod response;
pub mod transaction;
pub mod view;
