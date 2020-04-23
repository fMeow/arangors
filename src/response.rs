/// This module contains structures to deserialize responses from arangoDB
/// server via HTTP request, as well as convenient functions to deserialize
/// `Response`.
use std::fmt;
use std::{fmt::Debug, ops::Deref};

use failure::{format_err, Error as FailureError};
use log::trace;
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use serde_json::value::Value;

use super::aql::QueryStats;

/// There are different type of json object when requests to arangoDB
/// server is accepted or not. Here provides an abstraction for
/// response of success and failure.
///
/// When ArangoDB server response error code, then an error would be cast.
pub(crate) fn serialize_response<T>(text: &str) -> Result<T, FailureError>
where
    T: DeserializeOwned + Debug,
{
    let response: Response<T> = serde_json::from_str(text)?;
    response.into()
}

/// An enum to divide into successful and failed response.
///
/// Request to server can failed at application level, like insufficient
/// permission, database not found and etc. Response from arangoDB can tell
/// whether the query succeeded and why if it failed.
///
/// The function of this enum is almost the same as
/// Result, except that it's used to deserialize from
/// server response.
#[derive(Debug)]
pub enum Response<T> {
    Ok(Success<T>),
    Err(ServerError),
}

impl<T> Into<Result<T, FailureError>> for Response<T> {
    fn into(self) -> Result<T, FailureError> {
        match self {
            Response::Ok(success) => Ok(success.result),
            Response::Err(err) => Err(format_err!("{}", err.message)),
        }
    }
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = serde_json::Map::deserialize(deserializer)?;
        trace!("Deserialize normal Response: {:?}", map);
        let error = map
            .get("error")
            .ok_or_else(|| de::Error::missing_field("error"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if error {
            ServerError::deserialize(rest)
                .map(Response::Err)
                .map_err(de::Error::custom)
        } else {
            Success::<T>::deserialize(rest)
                .map(Response::Ok)
                .map_err(de::Error::custom)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ArangoResult<T> {
    #[serde(rename = "result")]
    result: T,
}

impl<T> ArangoResult<T> {
    pub fn unwrap(self) -> T {
        self.result
    }
}

impl<T> Deref for ArangoResult<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

#[derive(Deserialize, Debug)]
pub struct Success<T> {
    pub error: bool,
    pub code: u16,
    #[serde(flatten)]
    pub result: T,
}

impl<T: fmt::Display> fmt::Display for Success<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(format!("Response {} (Status: {})", &self.result, &self.code).as_str())
    }
}

#[derive(Deserialize, Debug)]
pub struct ServerError {
    pub(crate) code: u16,
    #[serde(rename = "errorNum")]
    pub(crate) error_num: u16,
    #[serde(rename = "errorMessage")]
    pub(crate) message: String,
}

impl ServerError {
    /// Get the HTTP status code of an error response.
    pub fn get_code(&self) -> u16 {
        self.code
    }

    pub fn get_error_num(&self) -> u16 {
        self.error_num
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[derive(Deserialize, Debug)]
pub struct Cursor<T> {
    /// the total number of result documents available
    ///
    /// only available if the query was executed with the count attribute
    /// set
    pub count: Option<usize>,
    /// a boolean flag indicating whether the query result was served from
    /// the query cache or not.
    ///
    /// If the query result is served from the query cache, the extra
    /// return attribute will not contain any stats sub-attribute
    /// and no profile sub-attribute.,
    pub cached: bool,
    /// A boolean indicator whether there are more results available for
    /// the cursor on the server
    #[serde(rename = "hasMore")]
    pub more: bool,

    /// (anonymous json object): an array of result documents (might be
    /// empty if query has no results)
    pub result: Vec<T>,
    ///  id of temporary cursor created on the server
    pub id: Option<String>,

    /// an optional JSON object with extra information about the query
    /// result contained in its stats sub-attribute. For
    /// data-modification queries, the extra.stats sub-attribute
    /// will contain the number of
    /// modified documents and the number of documents that could
    /// not be modified due to an error if ignoreErrors query
    /// option is specified.
    pub extra: Option<Extra>,
}

#[derive(Deserialize, Debug)]
pub struct Extra {
    // TODO
    stats: Option<QueryStats>,
    // TODO
    warnings: Option<Vec<Value>>,
}
