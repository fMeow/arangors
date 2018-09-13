#![feature(external_doc)]
#![doc(include = "../README.md")]

pub mod aql;
pub mod collection;
pub mod connection;
pub mod database;
pub mod document;
mod query;
pub mod response;
mod session;

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
