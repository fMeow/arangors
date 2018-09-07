use std::fmt::Debug;

use failure::{format_err, Error};

use log::{error, trace};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Success {
        error: bool,
        code: u8,
        result: T,
    },

    // TODO implement error trait
    Error {
        error: bool,
        code: u8,
        #[serde(rename = "errorNum")]
        error_num: u16,
        #[serde(rename = "errorMessage")]
        message: String,
    },
}

/// There are different type of json object when requests to arangoDB
/// server is accepted or not. Here provides an abstraction for
/// response of success and failure.
/// TODO more intuitive response error enum
pub fn serialize_response<T>(mut resp: reqwest::Response) -> Result<T, Error>
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
        Response::Success { result, .. } => Ok(result),
        Response::Error { message, .. } => Err(format_err!("{}", message)),
    }
}
// impl Error {
//     /// Construct an Error.
//     pub fn new<T: Into<String>>(status_code: u16, error_code: ErrorCode, message: T) -> Self {
//         Error {
//             code,
//             error_num,
//             message: message.into(),
//         }
//     }

//     /// Get the HTTP status code of an error response.
//     pub fn get_code(&self) -> u16 {
//         self.code
//     }

//     pub fn get_error_num(&self) -> u16 {
//         self.error_num
//     }

//     pub fn get_message(&self) -> &str {
//         &self.message
//     }
// }

// impl<T: fmt::Display> fmt::Display for Response<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Response::Success { code, result, .. } => {
//                 f.write_str(format!("Response {} (Status: {})", &result, &code).as_str())
//             }
//             Response::Error {
//                 code,
//                 error_num,
//                 message,
//                 ..
//             } => f.write_str(
//                 format!("Error {}: {} (Status: {})", &error_num, &message, &code).as_str(),
//             ),
//         }
//     }
// }
