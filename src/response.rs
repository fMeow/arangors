/// This module contains structures to deserialize responses from arangoDB
/// server via HTTP request, as well as convenient functions to deserialize
/// `Response`.
use std::fmt;
use std::fmt::Debug;

use failure::{format_err, Error as FailureError};

use log::{error, trace};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use serde_json::value::Value;

use super::aql::QueryStats;

pub(crate) fn get_cursor<T>(resp: reqwest::Response) -> Result<Query<T>, FailureError>
where
    T: DeserializeOwned + Debug,
{
    let response = serialize(resp)?;
    match response {
        // TODO handling AQL query result
        Response::Query(resp) => Ok(resp),
        Response::Error(error) => Err(format_err!("{}", error.message)),
        Response::Success(resp) => {
            error!("Response success but expect cursor: {:?}", resp);
            panic!("Use get_result instead method when not performing query request")
        }
    }
}

/// There are different type of json object when requests to arangoDB
/// server is accepted or not. Here provides an abstraction for
/// response of success and failure.
/// TODO more intuitive response error enum
pub(crate) fn get_result<T>(resp: reqwest::Response) -> Result<T, FailureError>
where
    T: DeserializeOwned + Debug,
{
    let response = serialize(resp)?;
    match response {
        Response::Success(resp) => Ok(resp.result),
        Response::Error(error) => Err(format_err!("{}", error.message)),
        // TODO handling AQL query result
        Response::Query(resp) => Ok(resp.result),
    }
}

fn serialize<T>(mut resp: reqwest::Response) -> Result<Response<T>, FailureError>
where
    T: DeserializeOwned + Debug,
{
    let response_text = resp.text()?;
    let result: Response<T> = serde_json::from_str(response_text.as_str()).map_err(|err| {
        error!(
            "Failed to serialize.\n\tResponse: {:?} \n\tText: {:?}",
            resp, response_text
        );
        err
    })?;
    Ok(result)
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
    Query(Query<T>),
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
pub struct Query<T> {
    error: bool,
    code: u8,

    /// the total number of result documents available
    ///
    ///  only available if the query was executed with the count attribute
    /// set
    count: Option<usize>,
    /// a boolean flag indicating whether the query result was served from
    /// the query cache or not.
    ///
    /// If the query result is served from the query cache, the extra
    /// return attribute will not contain any stats sub-attribute
    /// and no profile sub-attribute.,
    cached: bool,
    /// A boolean indicator whether there are more results available for
    /// the cursor on the server
    #[serde(rename = "hasMore")]
    more: bool,

    /// (anonymous json object): an array of result documents (might be
    /// empty if query has no results)
    result: T,
    ///  id of temporary cursor created on the server
    id: Option<String>,

    /// an optional JSON object with extra information about the query
    /// result contained in its stats sub-attribute. For
    /// data-modification queries, the extra.stats sub-attribute
    /// will contain the number of
    /// modified documents and the number of documents that could
    /// not be modified due to an error if ignoreErrors query
    /// option is specified.
    extra: Option<Extra>,
}
impl<T: fmt::Display> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(format!("Response {} (Status: {})", &self.result, &self.code).as_str())
    }
}

#[derive(Deserialize, Debug)]
struct Extra {
    // TODO
    stats: Option<QueryStats>,
    // TODO
    warnings: Option<Vec<Value>>,
}
