//! # arangors
//!
//! [![Build Status](https://travis-ci.org/fMeow/arangors.svg?branch=master)](https://travis-ci.org/fMeow/arangors)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/arangors.svg)](https://crates.io/crates/arangors)
//! [![arangors](https://docs.rs/arangors/badge.svg)](https://docs.rs/arangors)
//!
//! `arangors` is an intuitive rust client for [arangoDB](https://www.arangodb.com/),
//! inspired by [pyArango](https://github.com/tariqdaouda/pyArango).
//!
//! `arangors` enables you to connect with arangoDB server, access to database,
//! execute AQL query, manage arangoDB in an easy and intuitive way.
//!
//! ## Philosophy of arangors
//!
//! `arangors` is targeted at ergonomic, intuitive and OOP-like API for
//! ArangoDB, both top level and low level API for users' choice.
//!
//! Overall architecture of arangoDB:
//!
//! > databases -> collections -> documents/edges
//!
//! In fact, the design of `arangors` just mimic this architecture, with a
//! slight difference that in the top level, there is a connection object on top
//! of databases, containing a HTTP client with authentication information in
//! HTTP headers.
//!
//! Hierarchy of arangors:
//! > connection -> databases(cached) -> collections -> documents/edges
//!
//! ## Features
//!
//! By now, the available features of arangors are:
//!
//! - make connection to arangoDB
//! - get list of databases and collections
//! - full featured AQL query
//!
//! ## TODO
//!
//! - (Done) Milestone 0.1.x
//!
//! Synchronous connection based on `reqwest` and full featured AQL query.
//!
//! - (WIP) Milestone 0.2.x
//!
//! Remove cache behaviour and fix severe bugs in 0.2 that only root user can
//! have access to arangoDB, which impose breaking API changes.
//!
//! Fill the unimplemented API in `Connection`, `Database`, `Collection` and
//! `Document`.
//!
//! In this stage, all operations available for database, collection and
//! document should be implemented.
//!
//! - Milestone 0.3.x
//!
//! Implement both sync and async client.
//!
//! - Milestone 0.4.x
//!
//! Provides the API related to graph, index and user management.
//!
//!
//! ## Glance
//!
//! ### Connection
//!
//! There is three way to establish connections:
//!
//! - jwt
//! - basic auth
//! - no authentication
//!
//! So are the `arangors` API.
//!
//! When a connection is successfully established,
//! `arangors` will automatically fetch the structure of arangoDB
//! by get the list of database, and then lists of collections per database.
//!
//! Example:
//!
//! - With authentication
//!
//! ```rust
//! use arangors::Connection;
//!
//! # #[cfg_attr(any(feature="reqwest_async", feature="surf_async"), maybe_async::maybe_async, tokio::main)]
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
//! # #[cfg_attr(any(feature="reqwest_async", feature="surf_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! let db = conn.db("test_db").await.unwrap();
//! let collection = db.collection("test_collection").await.unwrap();
//! # }
//! ```
//!
//! ### AQL Query
//!
//! All aql query related functions are associated with database, as AQL query
//! is performed at database level.
//!
//! There are several way to execute AQL query, and can be categorized into two
//! classes:
//!
//! - batch query
//!
//!     - `aql_query_batch`
//!     - `aql_next_batch`
//!
//! - query to fetch all results
//!
//!     - `aql_str`
//!     - `aql_bind_vars`
//!     - `aql_query`
//!
//! This later category provides a convenient high level API, whereas batch
//! query offers more control.
//!
//! #### Typed or Not Typed
//!
//! Note that results can be strong typed given deserializable struct, or
//! arbitrary JSON object with `serde::Value`.
//!
//! - Arbitrary JSON object
//!
//! ```rust
//! use arangors::Connection;
//! use serde_json::Value;
//!
//! # #[cfg_attr(any(feature="reqwest_async", feature="surf_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! let db = conn.db("test_db").await.unwrap();
//! let resp: Vec<Value> = db
//!     .aql_str("FOR u IN test_collection LIMIT 3 RETURN u")
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! - Strong typed result
//!
//! ```rust
//! use arangors::Connection;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! struct User {
//!     pub username: String,
//!     pub password: String,
//! }
//!
//! # #[cfg_attr(any(feature="reqwest_async", feature="surf_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! let db = conn.db("test_db").await.unwrap();
//! let resp: Vec<User> = db
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
//! #### Fetch All Results
//!
//! There are three functions for AQL query that fetch all results from
//! ArangoDB. These functions internally fetch batch results one after another
//! to get all results.
//!
//! The functions for fetching all results are listed as bellow:
//!
//! - `aql_str`
//!
//! This function only accept a AQL query string.
//!
//! Here is an example of strong typed query result with `aql_str`:
//!
//! ```rust
//! use arangors::Connection;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! struct User {
//!     pub username: String,
//!     pub password: String,
//! }
//!
//! # #[cfg_attr(any(feature="reqwest_async", feature="surf_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! let db = conn.db("test_db").await.unwrap();
//! let result: Vec<User> = db
//!     .aql_str(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! - `aql_bind_vars`
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
//! # #[cfg_attr(any(feature="reqwest_async", feature="surf_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! let db = conn.db("test_db").await.unwrap();
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
//! - `aql_query`
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
//! # #[cfg_attr(any(feature="reqwest_async", feature="surf_async"), maybe_async::maybe_async, tokio::main)]
//! # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
//! # async fn main() {
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
//!     .await
//!     .unwrap();
//! let database = conn.db("test_db").await.unwrap();
//!
//! let aql = AqlQuery::new("FOR u IN @@collection LIMIT 3 RETURN u")
//!     .batch_size(1)
//!     .count(true)
//!     .bind_var("@collection", "test_collection");
//!
//! let resp: Vec<Value> = database.aql_query(aql).await.unwrap();
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
//! An ergonomic [arangoDB](https://www.arangodb.com/) client for rust.
#![allow(unused_parens)]

#[cfg(any(
    feature = "reqwest_async",
    feature = "reqwest_blocking",
    feature = "surf_async"
))]
pub use crate::connection::Connection;
pub use crate::{
    aql::{AqlOption, AqlQuery},
    collection::Collection,
    connection::GenericConnection,
    database::Database,
    document::Document,
    error::{ArangoError, ClientError},
    response::{Cursor, Success},
};

pub mod aql;
pub mod client;
pub mod collection;
pub mod connection;
pub mod database;
pub mod document;
pub mod error;
mod query;
pub mod response;
