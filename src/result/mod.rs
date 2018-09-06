use serde_derive::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Success {
        error: bool,
        code: u16,
        result: T,
    },

    Error {
        error: bool,
        code: u8,
        #[serde(rename = "errorNum")]
        error_num: u16,
        #[serde(rename = "errorMessage")]
        message: String,
    },
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
