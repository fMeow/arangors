//! types to deserialize responses from arangoDB server via HTTP request, as
//! well as convenient functions to deserialize `Response`.
//!
//! For response with `error` and `code` fields indicating the whether the
//! request is successful, use `deserialize_response` to abstract over request
//! status and data of concerns.
//!
//! For response storing all information in `result` filed, use
//! `ArangoResult`.
use std::ops::Deref;

use log::trace;
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use serde_json::value::Value;

use crate::{ArangoError, ClientError};

/// Deserialize response from arango server
///
/// There are different type of json object when requests to arangoDB
/// server is accepted or not. Here provides an abstraction for
/// response of success and failure.
///
/// When ArangoDB server response error code, then an error would be cast.
pub(crate) fn deserialize_response<T>(text: &str) -> Result<T, ClientError>
where
    T: DeserializeOwned,
{
    let response: Response<T> = serde_json::from_str(text)?;
    Ok(Into::<Result<T, ArangoError>>::into(response)?)
}

/// An helper enum to divide into successful and failed response
///
/// Request to server can failed at application level, like insufficient
/// permission, database not found and etc. Response from arangoDB can tell
/// whether the query succeeded and why if it failed.
///
/// The function of this enum is almost the same as `Result`, except that it's
/// used to deserialize from server response. This enum is to facilitate
/// deserialization and it should be converted to `Result<T, ArangoError>`
/// eventually.
#[derive(Debug)]
pub(crate) enum Response<T> {
    Ok(T),
    Err(ArangoError),
}

impl<T> Into<Result<T, ArangoError>> for Response<T> {
    fn into(self) -> Result<T, ArangoError> {
        match self {
            Response::Ok(success) => Ok(success),
            Response::Err(err) => Err(err),
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
            .map_or_else(|| Ok(false), Deserialize::deserialize)
            .map_err(de::Error::custom)?;
        let rest = Value::Object(map);

        if error {
            ArangoError::deserialize(rest)
                .map(Response::Err)
                .map_err(de::Error::custom)
        } else {
            T::deserialize(rest)
                .map(Response::Ok)
                .map_err(de::Error::custom)
        }
    }
}

/// Helper struct to deserialize json result that store
/// information in "result" field
#[derive(Deserialize, Debug)]
pub(crate) struct ArangoResult<T> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct CollectionResponse {
        pub id: String,
        pub name: String,
        pub status: u8,
        pub r#type: u8,
        #[serde(rename = "isSystem")]
        pub is_system: bool,
    }

    #[test]
    fn response() {
        let text = "{\"id\":\"9947\",\"name\":\"relation\",\"status\":2,\"type\":3,\"isSystem\": \
                    false,\"globallyUniqueId\":\"hD260BE2A30F9/9947\"}";
        let result = serde_json::from_str::<Response<CollectionResponse>>(text);
        assert_eq!(result.is_ok(), true, "failed: {:?}", result);

        let text = "{\"error\":false,\"code\":412,\"id\":\"9947\",\"name\":\"relation\",\"status\"\
                    :2,\"type\":3,\"isSystem\": false,\"globallyUniqueId\":\"hD260BE2A30F9/9947\"}";
        let result = serde_json::from_str::<Response<CollectionResponse>>(text);
        assert_eq!(result.is_ok(), true, "failed: {:?}", result);

        let text = "{\"error\":true,\"code\":412,\"errorMessage\":\"error\",\"errorNum\":1200}";
        let result = serde_json::from_str::<Response<CollectionResponse>>(text);
        assert_eq!(result.is_ok(), true, "failed: {:?}", result);
        let response = Into::<Result<_, _>>::into(result.unwrap());

        assert_eq!(
            response.is_err(),
            true,
            "response should be error: {:?}",
            response
        );
    }
}
