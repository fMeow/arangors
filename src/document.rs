use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    _id: String,
    _key: String,
    _rev: String,

    #[serde(flatten)]
    pub document: T,
}
