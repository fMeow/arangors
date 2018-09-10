/// This module contains structures to deserialize responses from arangoDB
/// server via HTTP request, as well as convenient functions to deserialize
/// `Response`.
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use failure::{format_err, Error as FailureError};

use url::Url;
use log::{error, trace};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use serde_json::value::Value;
use reqwest::Client;

use super::aql::QueryStats;
use super::database::Database;

pub(crate) fn serialize_query_response<T>(mut resp: reqwest::Response) -> Result<Cursor<T>, FailureError>
    where
        T: DeserializeOwned + Debug,
{
    let response_text = resp.text()?;
    let response: QueryResponse<T> = serde_json::from_str(response_text.as_str()).map_err(|err| {
        error!(
            "Failed to serialize.\n\tResponse: {:?} \n\tText: {:?}",
            resp, response_text
        );
        err
    })?;
    match response {
        QueryResponse::Success(resp) => Ok(resp),
        QueryResponse::Error(error) => Err(format_err!("{}", error.message)),
    }
}

/// There are different type of json object when requests to arangoDB
/// server is accepted or not. Here provides an abstraction for
/// response of success and failure.
/// TODO more intuitive response error enum
pub(crate) fn serialize_response<T>(mut resp: reqwest::Response) -> Result<T, FailureError>
    where
        T: DeserializeOwned + Debug,
{
    let response_text = resp.text()?;
    let response: Response<T> = serde_json::from_str(response_text.as_str()).map_err(|err| {
        error!(
            "Failed to serialize.\n\tResponse: {:?} \n\tText: {:?}",
            resp, response_text
        );
        err
    })?;
    match response {
        Response::Success(resp) => Ok(resp.result),
        Response::Error(error) => Err(format_err!("{}", error.message)),
    }
}

/// A enum of response contains all the case clients will encounter:
/// - Query result (Cursor)
/// - Error
/// - successful request but not query result
///
/// Never transpose the order of `Query` and `Success` as serde deserialize
/// response in order. And `Query` is just a super set of `Success`
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum QueryResponse<T> {
    Success(Cursor<T>),
    Error(Error),
}

/// A enum of response contains all the case clients will encounter:
/// - Query result (Cursor)
/// - Error
/// - successful request but not query result
///
/// Never transpose the order of `Query` and `Success` as serde deserialize
/// response in order. And `Query` is just a super set of `Success`
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Success(Success<T>),
    Error(Error),
}

#[derive(Deserialize, Debug)]
pub struct Success<T> {
    error: bool,
    code: u8,
    result: T,
}

impl<T: fmt::Display> fmt::Display for Success<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(format!("Response {} (Status: {})", &self.result, &self.code).as_str())
    }
}

#[derive(Deserialize, Debug)]
pub struct Error {
    error: bool,
    code: u8,
    #[serde(rename = "errorNum")]
    error_num: u16,
    #[serde(rename = "errorMessage")]
    message: String,
}

impl Error {
    /// Get the HTTP status code of an error response.
    pub fn get_code(&self) -> u8 {
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
    pub error: bool,

    /// HTTP status code
    pub code: u8,

    /// the total number of result documents available
    ///
    ///  only available if the query was executed with the count attribute
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
