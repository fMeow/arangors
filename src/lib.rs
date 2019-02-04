//! An ergonomic [arangoDB](https://www.arangodb.com/) client for rust.
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
