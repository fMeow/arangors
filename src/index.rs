use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Fulltext,
    Geo,
    Hash,
    Persistent,
    Skiplist,
    Ttl,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Index {
    fields: Vec<String>,
    name: Option<String>,
    unique: Option<bool>,
    r#type: Type,
    sparse: Option<bool>,
    min_length: Option<u32>,
    deduplicate: Option<bool>,
    geojson: Option<bool>,
    expire_after: Option<u32>,
}

impl Index {
    fn default(r#type: Type) -> Self {
        Index {
            fields: vec![],
            name: None,
            unique: None,
            r#type,
            sparse: None,
            min_length: None,
            deduplicate: None,
            geojson: None,
            expire_after: None,
        }
    }
    pub fn persistent(fields: Vec<String>, unique: bool, sparse: bool) -> Self {
        let mut index = Index::default(Type::Persistent);
        index.fields = fields;
        index.unique = Some(unique);
        index.sparse = Some(sparse);
        index
    }

    pub fn hash(fields: Vec<String>, unique: bool, sparse: bool, deduplicate: bool) -> Self {
        let mut index = Index::default(Type::Hash);
        index.fields = fields;
        index.unique = Some(unique);
        index.sparse = Some(sparse);
        index.deduplicate = Some(deduplicate);
        index
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
}