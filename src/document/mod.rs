use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};

pub use options::*;

mod options;

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHeader {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
}

/// Structure that represents a document within its content and header
#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    #[serde(flatten)]
    pub header: DocumentHeader,
    #[serde(flatten)]
    pub document: T,
}

impl<'de, T> Document<T>
where
    T: Serialize + Deserialize<'de>,
{
    pub fn new(data: T) -> Self {
        Document {
            document: data,
            header: DocumentHeader {
                _id: String::new(),
                _key: String::new(),
                _rev: String::new(),
            },
        }
    }
}

/// Standard Response when having CRUD operation on document
/// TODO could add more response variant as shown in official doc
/// 200: is returned if the document was found
/// 304: is returned if the “If-None-Match” header is given and the document has
/// the same version 404: is returned if the document or collection was not
/// found 412: is returned if an “If-Match” header is given and the found
/// document has a different version. The response will also contain the found
/// document’s current revision in the Etag header.
pub enum DocumentResponse<T> {
    /// Silent is when there is empty object returned by the server
    Silent,
    /// Contain data after CRUD
    Response {
        header: DocumentHeader,
        old: Option<T>,
        new: Option<T>,
        _old_rev: Option<String>,
    },
}

/// Gives extra method on the DocumentResponse to quickly check what the server
/// returns
impl<T> DocumentResponse<T> {
    /// Should be true when the server send back an empty object {}
    pub fn is_silent(&self) -> bool {
        match self {
            DocumentResponse::Silent => true,
            _ => false,
        }
    }
    /// Should be true if there is a response from the server
    pub fn has_response(&self) -> bool {
        match self {
            DocumentResponse::Response { .. } => true,
            _ => false,
        }
    }

    /// Return the document header contained inside the response
    pub fn header(&self) -> Option<&DocumentHeader> {
        if let DocumentResponse::Response { header, .. } = self {
            Some(header)
        } else {
            None
        }
    }
    /// Return the old document before changes
    pub fn old_doc(&self) -> Option<&T> {
        Option::from(if let DocumentResponse::Response { old, .. } = self {
            old
        } else {
            &None
        })
    }
    /// Return the new document
    pub fn new_doc(&self) -> Option<&T> {
        Option::from(if let DocumentResponse::Response { new, .. } = self {
            new
        } else {
            &None
        })
    }
    /// return the old revision of the document
    pub fn old_rev(&self) -> Option<&String> {
        Option::from(if let DocumentResponse::Response { _old_rev, .. } = self {
            _old_rev
        } else {
            &None
        })
    }
}

impl<'de, T> Deserialize<'de> for DocumentResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut obj = serde_json::Value::deserialize(deserializer)?;

        let json = obj.as_object_mut().unwrap();

        if json.contains_key("_key") != true {
            Ok(DocumentResponse::Silent)
        } else {
            let header: DocumentHeader = DocumentHeader {
                _id: serde_json::from_value(json.remove("_id").unwrap()).unwrap(),
                _key: serde_json::from_value(json.remove("_key").unwrap()).unwrap(),
                _rev: serde_json::from_value(json.remove("_rev").unwrap()).unwrap(),
            };

            let old = if json.contains_key("old") {
                T::deserialize(json.remove("old").unwrap()).ok()
            } else {
                None
            };

            let new = if json.contains_key("new") {
                T::deserialize(json.remove("new").unwrap()).ok()
            } else {
                None
            };
            let _old_rev = if json.contains_key("_old_rev") {
                Some(json.remove("_old_rev").unwrap().to_string())
            } else {
                None
            };

            Ok(DocumentResponse::Response {
                header,
                old,
                new,
                _old_rev,
            })
        }
    }
}
