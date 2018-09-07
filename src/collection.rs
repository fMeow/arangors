use failure::Error;
use std::rc::Rc;

use reqwest::{Client, Url};
use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde_derive::Deserialize;

use super::database::Database;

#[derive(Debug)]
pub struct Collection {
    id: String,
    name: String,
    collection_type: CollectionType,
    base_url: Url,
    session: Rc<Client>,
}
impl<'a, 'b: 'a> Collection {
    pub fn new<T: Into<String>>(
        database: &'b Database,
        name: T,
        id: T,
        collection_type: CollectionType,
    ) -> Result<Collection, Error> {
        let url = database.get_url().join("collection")?;
        Ok(Collection {
            name: name.into(),
            id: id.into(),
            session: database.get_session(),
            base_url: url,
            collection_type,
        })
    }
    pub fn from_response(
        database: &'b Database,
        collection: &CollectionResponse,
    ) -> Result<Collection, Error> {
        let url = database.get_url().join("collection")?;
        Ok(Collection {
            id: collection.id.to_owned(),
            name: collection.name.to_owned(),
            collection_type: collection.collection_type.clone(),
            session: database.get_session(),
            base_url: url,
        })
    }
    pub fn get_collection_type(&self) -> &CollectionType {
        &self.collection_type
    }
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_url(&self) -> &Url {
        &self.base_url
    }

    pub fn get_session(&self) -> Rc<Client> {
        Rc::clone(&self.session)
    }
}

#[derive(Debug, Deserialize)]
pub struct CollectionResponse {
    pub id: String,
    pub name: String,
    pub status: CollectionStatus,
    #[serde(rename = "type")]
    pub collection_type: CollectionType,
    #[serde(rename = "isSystem")]
    pub is_system: bool,
    // #[serde(rename = "globallyUniqueId")]
    // pub global_unique_id: String,
}

#[derive(Debug)]
pub enum CollectionStatus {
    NewBorn,
    Unloaded,
    Loaded,
    BeingUnload,
    Deleted,
    Loading,
}
impl<'de> Deserialize<'de> for CollectionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            1 => Ok(CollectionStatus::NewBorn),
            2 => Ok(CollectionStatus::Unloaded),
            3 => Ok(CollectionStatus::Loaded),
            4 => Ok(CollectionStatus::BeingUnload),
            5 => Ok(CollectionStatus::Deleted),
            6 => Ok(CollectionStatus::Loading),
            _ => Err(DeError::custom("Undefined behavior. If the crate breaks after a upgrade of ArangoDB, please contact the author.")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CollectionType {
    Document,
    Edge,
}
impl<'de> Deserialize<'de> for CollectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            2 => Ok(CollectionType::Document),
            3 => Ok(CollectionType::Edge),
            _ => Err(DeError::custom("Undefined behavior. If the crate breaks after a upgrade of ArangoDB, please contact the author.")),
        }
    }
}
