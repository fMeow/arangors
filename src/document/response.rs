//! Types of response related to document
use serde::{de::Error as DeError, Deserialize, Deserializer};

use super::Header;

/// Standard Response when having CRUD operation on document
///
/// TODO could add more response variant as shown in official doc
///
/// 200: is returned if the document was found
///
/// 304: is returned if the “If-None-Match” header is given and the document has
/// the same version
///
/// 404: is returned if the document or collection was not
/// found
///
/// 412: is returned if an “If-Match” header is given and the found
/// document has a different version. The response will also contain the found
/// document’s current revision in the Etag header.
pub enum DocumentResponse<T> {
    /// Silent is when there is empty object returned by the server
    Silent,
    /// Contain data after CRUD
    Response {
        header: Header,
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
        matches!(self, DocumentResponse::Silent)
    }
    /// Should be true if there is a response from the server
    pub fn has_response(&self) -> bool {
        matches!(self, DocumentResponse::Response { .. })
    }

    /// Return the document header contained inside the response
    pub fn header(&self) -> Option<&Header> {
        if let DocumentResponse::Response { header, .. } = self {
            Some(header)
        } else {
            None
        }
    }
    /// Return the old document before changes
    pub fn old_doc(&self) -> Option<&T> {
        if let DocumentResponse::Response { old, .. } = self {
            old.as_ref()
        } else {
            None
        }
    }
    /// Return the new document
    pub fn new_doc(&self) -> Option<&T> {
        if let DocumentResponse::Response { new, .. } = self {
            new.as_ref()
        } else {
            None
        }
    }
    /// return the old revision of the document
    pub fn old_rev(&self) -> Option<&String> {
        if let DocumentResponse::Response { _old_rev, .. } = self {
            _old_rev.as_ref()
        } else {
            None
        }
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

        let json = obj
            .as_object_mut()
            .ok_or_else(|| DeError::custom("should be a json object"))?;

        if json.is_empty() {
            Ok(DocumentResponse::Silent)
        } else {
            let _id = json
                .remove("_id")
                .ok_or_else(|| DeError::missing_field("_id"))?;
            let _key = json
                .remove("_key")
                .ok_or_else(|| DeError::missing_field("_key"))?;
            let _rev = json
                .remove("_rev")
                .ok_or_else(|| DeError::missing_field("_rev"))?;
            let header: Header = Header {
                _id: serde_json::from_value(_id).map_err(DeError::custom)?,
                _key: serde_json::from_value(_key).map_err(DeError::custom)?,
                _rev: serde_json::from_value(_rev).map_err(DeError::custom)?,
            };

            let old = json
                .remove("old")
                .map(T::deserialize)
                .transpose()
                .map_err(DeError::custom)?;
            let new = json
                .remove("new")
                .map(T::deserialize)
                .transpose()
                .map_err(DeError::custom)?;
            let _old_rev = json.remove("_old_rev").map(|v| v.to_string());

            Ok(DocumentResponse::Response {
                header,
                old,
                new,
                _old_rev,
            })
        }
    }
}
