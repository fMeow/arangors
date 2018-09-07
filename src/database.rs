use failure::Error;
use std::collections::HashMap;
use std::rc::Rc;

use reqwest::{Client, Url};
use serde_derive::Deserialize;

use super::collection::{Collection, CollectionResponse};
use super::connection::Connection;
use super::response::{serialize_response, Response};

#[derive(Debug)]
pub struct Database {
    name: String,
    base_url: Url,
    session: Rc<Client>,
    collections: HashMap<String, Collection>,
    system_collections: HashMap<String, Collection>,
}
impl<'a, 'b: 'a> Database {
    pub fn new<T: Into<String>>(conn: &'b Connection, name: T) -> Result<Database, Error> {
        let name = name.into();
        let path = format!("/_db/{}/_api", name.as_str());
        let url = conn.get_url().join(path.as_str())?;
        let mut database = Database {
            name,
            session: conn.get_session(),
            base_url: url,
            collections: HashMap::new(),
            system_collections: HashMap::new(),
        };
        database.retrieve_collections()?;
        Ok(database)
    }
    /// The last steps of connection establishment is to query the accessible
    /// databases and cache them in a hashmap of `Databases` objects.
    ///
    /// 1. retrieve the names of all the accessible databases
    /// 1. for each databases, construct a `Database` object and store them in
    /// `self.databases` for later use
    ///
    /// This function uses the API that is used to retrieve a list of
    /// all databases the current user can access.
    fn retrieve_collections(&mut self) -> Result<&mut Database, Error> {
        // an invalid arango_url should never running through initialization
        // so we assume arango_url is a valid url
        // When we pass an invalid path, it should panic to eliminate the bug
        // in development.
        let url = self.base_url.join("collection").unwrap();
        let resp = self.session.get(url).send()?;
        let result: Vec<CollectionResponse> = serialize_response(resp)?;
        for coll in result.iter() {
            let collection = Collection::from_response(self, coll)?;
            if coll.is_system {
                self.system_collections
                    .insert(coll.name.to_owned(), collection);
            } else {
                self.system_collections
                    .insert(coll.name.to_owned(), collection);
            }
        }
        Ok(self)
    }

    pub fn get_url(&self) -> &Url {
        &self.base_url
    }

    pub fn get_session(&self) -> Rc<Client> {
        Rc::clone(&self.session)
    }
}
