use failure::Error;
use std::collections::HashMap;
use std::rc::Rc;

use log::{info, trace};
use reqwest::{Client, Url};

use super::collection::{Collection, CollectionResponse};
use super::connection::Connection;
use super::response::Query;
use super::response::{get_cursor, get_result};

#[derive(Debug)]
pub struct Database {
    name: String,
    base_url: Url,
    session: Rc<Client>,
    collections: HashMap<String, Collection>,
    system_collections: HashMap<String, Collection>,
}
impl<'a, 'b: 'a> Database {
    ///
    ///
    ///
    ///  Base url should be like `http://localhost:8529/`
    pub fn new<T: Into<String>>(conn: &'b Connection, name: T) -> Result<Database, Error> {
        let name = name.into();
        let path = format!("/_db/{}/_api/", name.as_str());
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
    /// Retrieve all collections of this database.
    ///
    /// 1. retrieve the names of all collections
    /// 1. cache colelctions
    ///     - for user collection, construct a `Collection` object and store them in
    /// `self.collections` for later use
    ///     - for system collection, construct a `Collection` object and store them in
    /// `self.system_collections` for later use
    fn retrieve_collections(&mut self) -> Result<&mut Database, Error> {
        // an invalid arango_url should never running through initialization
        // so we assume arango_url is a valid url
        // When we pass an invalid path, it should panic to eliminate the bug
        // in development.
        let url = self.base_url.join("collection").unwrap();
        trace!(
            "Retrieving collections from {:?}: {}",
            self.name,
            url.as_str()
        );
        let resp = self.session.get(url).send()?;
        let result: Vec<CollectionResponse> =
            serialize_response(resp).expect("Failed to serialize Collection response");
        trace!("Collections retrieved");

        for coll in result.iter() {
            let collection = Collection::from_response(self, coll)?;
            if coll.is_system {
                // trace!("System collection: {:?}", coll.name);
                self.system_collections
                    .insert(coll.name.to_owned(), collection);
            } else {
                trace!("Collection: {:?}", coll.name);
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

    /// Get collection object with name.
    ///
    /// This function look up user collections in cache hash map,
    /// and return a reference of collection if found.
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        match self.collections.get(name) {
            Some(database) => Some(&database),
            None => {
                info!("User collection {} not found.", name);
                None
            }
        }
    }
    /// Get system collection object with name.
    ///
    /// This function look up system collections in cache hash map,
    /// and return a reference of collection if found.
    pub fn get_system_collection(&self, name: &str) -> Option<&Collection> {
        match self.system_collections.get(name) {
            Some(database) => Some(&database),
            None => {
                info!("User collection {} not found.", name);
                None
            }
        }
    }

    pub fn has_collection(&self, name: &str) -> bool {
        let system = match self.get_system_collection(name) {
            Some(_) => true,
            None => false,
        };
        let user = match self.get_collection(name) {
            Some(_) => true,
            None => false,
        };
        user | system
    }

    pub fn has_user_collection(&self, name: &str) -> bool {
        match self.get_collection(name) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn has_system_collection(&self, name: &str) -> bool {
        match self.get_system_collection(name) {
            Some(_) => true,
            None => false,
        }
    }
}
