//! A intuitive rust client for [arangoDB](https://www.arangodb.com/),
//! inspired by [pyArango](https://github.com/tariqdaouda/pyArango).

pub mod aql;
pub mod collection;
pub mod connection;
pub mod database;
mod query;
pub mod response;

pub use crate::collection::Collection;
pub use crate::connection::Connection;
pub use crate::database::Database;

pub use crate::response::Cursor;
pub use crate::response::Error;
pub use crate::response::Response;
pub use crate::response::Success;

pub use crate::aql::AqlOption;
pub use crate::aql::AqlQuery;
