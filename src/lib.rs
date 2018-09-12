//! `Arangors` is an intuitive rust client for [arangoDB](https://www.arangodb.com/),
//! inspired by [pyArango](https://github.com/tariqdaouda/pyArango).
//!
//! Arangors enables you to connect with arangoDB server, access to database,
//! collection and documents in an easy and intuitive way.
//!
//! ## NOTICE
//! `Arangors` is targeted at `Rust 2018`, so this driver would be remains in
//! nightly channel until the release of `Rust 2018`.
//!
//! Also, `arangors` will stay synchronous until the `futures ` crate reach
//! `1.0`.
//!
//!
//! ## Connection
//! There is three way to establish connections:
//! - jwt
//! - basic auth
//! - no authentication
//!
//! So are the `arangors` API:
//! Example:
//!
//! ```rust,ignore
//! use arangors::Connection;
//!
//! // (Recommended) Handy functions
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
//! let conn =
//!     Connection::establish_basic_auth("http://localhost:8529", "username", "password").unwrap();
//! let conn = Connection::establish_without_auth("http://localhost:8529", "username", "password")
//!     .unwrap();
//! ```
//!
//! ## AQL Query
//! Example:
//!
//! ```rust,ignore
//! use
//! use arangors::{AqlQuery, Connection, Cursor, Database};
//! use serde_json::value::Value;
//!
//! fn main() {
//!     pretty_env_logger::init();
//!
//!     let conn = Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
//!     let database = conn.get_database("database").unwrap();
//!
//!     let aql: AqlQuery<Value> = AqlQuery {
//!         query: "FOR u IN Collection LIMIT 3 RETURN u",
//!         batch_size: Some(1),
//!         ..Default::default()
//!     };
//!     let resp: Vec<Value> = database.aql_query(aql).unwrap();
//!     println!("{:?}", resp);
//! }
//! ```

pub mod aql;
pub mod collection;
pub mod connection;
pub mod database;
pub mod document;
mod query;
pub mod response;

pub use crate::collection::Collection;
pub use crate::connection::Connection;
pub use crate::database::Database;
pub use crate::document::Document;

pub use crate::response::Cursor;
pub use crate::response::Error;
pub use crate::response::Response;
pub use crate::response::Success;

pub use crate::aql::AqlOption;
pub use crate::aql::AqlQuery;
