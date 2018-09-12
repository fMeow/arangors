/// This module contains structures to deserialize responses from arangoDB
/// server via HTTP request, as well as convenient functions to deserialize
/// `Response`.
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use failure::{format_err, Error as FailureError};

use log::{error, trace};
use reqwest::Client;
use serde::de::{self, Deserialize, DeserializeOwned, Deserializer};
use serde_derive::Deserialize;
use serde_json::value::Value;
use url::Url;

use super::aql::QueryStats;
use super::database::Database;

pub(crate) fn serialize_query_response<T>(
    mut resp: reqwest::Response,
) -> Result<Cursor<T>, FailureError>
where
    T: DeserializeOwned + Debug,
{
    let response_text = resp.text()?;
    let response: QueryResponse<T> =
        serde_json::from_str(response_text.as_str()).map_err(|err| {
            error!(
                "Failed to serialize.\n\tResponse: {:?} \n\tText: {:?}",
                resp, response_text
            );
            err
        })?;
    match response {
        QueryResponse::Ok(resp) => Ok(resp),
        QueryResponse::Err(error) => Err(format_err!("{}", error.message)),
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
        Response::Ok(resp) => Ok(resp.result),
        Response::Err(error) => Err(format_err!("{}", error.message)),
    }
}

/// A enum of response contains all the case clients will encounter:
/// - Query result (Cursor)
/// - Error
/// - successful request but not query result
///
/// Never transpose the order of `Query` and `Success` as serde deserialize
/// response in order. And `Query` is just a super set of `Success`
#[derive(Debug)]
pub enum QueryResponse<T> {
    Ok(Cursor<T>),
    Err(Error),
}

impl<'de, T> Deserialize<'de> for QueryResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        trace!("Deserialize QueryResponse: {:?}", map);
        let error = map
            .get("error")
            .ok_or_else(|| de::Error::missing_field("error"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if error {
            Error::deserialize(rest)
                .map(QueryResponse::Err)
                .map_err(de::Error::custom)
        } else {
            Cursor::<T>::deserialize(rest)
                .map(QueryResponse::Ok)
                .map_err(de::Error::custom)
        }
    }
}

/// A enum of response contains all the case clients will encounter:
/// - Query result (Cursor)
/// - Error
/// - successful request but not query result
///
/// Never transpose the order of `Query` and `Success` as serde deserialize
/// response in order. And `Query` is just a super set of `Success`
#[derive(Debug)]
pub enum Response<T> {
    Ok(Success<T>),
    Err(Error),
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        trace!("Deserialize normal Response: {:?}", map);
        let error = map
            .get("error")
            .ok_or_else(|| de::Error::missing_field("error"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if error {
            Error::deserialize(rest)
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
pub struct Success<T> {
    error: bool,
    code: u16,
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
    code: u16,
    #[serde(rename = "errorNum")]
    error_num: u16,
    #[serde(rename = "errorMessage")]
    message: String,
}

impl Error {
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
    pub error: bool,

    /// HTTP status code
    pub code: u16,

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
