//! database contains all struct and enum pertain to arangoDB "database" level.
//!
//! AQL query are all executed in database level, so Database offers AQL query.
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use failure::{format_err, Error};
use log::{debug, info, trace};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::Value;
use url::Url;

use super::aql::AqlQuery;
use super::collection::{Collection, CollectionResponse};
use super::connection::Connection;
use super::response::Cursor;
use super::response::{
    serialize_query_response, serialize_response, try_serialize_response, Response,
};

#[derive(Debug)]
pub struct Database {
    name: String,
    base_url: Url,
    session: Arc<Client>,
    collections: HashMap<String, Collection>,
    system_collections: HashMap<String, Collection>,
}

impl<'a, 'b: 'a> Database {
    ///  Base url should be like `http://localhost:8529/`
    pub fn new<T: Into<String>>(conn: &'b Connection, name: T) -> Result<Database, Error> {
        let name = name.into();
        let path = format!("/_db/{}/_api/", name.as_str());
        let url = conn.get_url().join(path.as_str()).unwrap();
        let mut database = Database {
            name,
            session: conn.get_session(),
            base_url: url,
            collections: HashMap::new(),
            system_collections: HashMap::new(),
        };
        database.fetch_collections()?;
        Ok(database)
    }
    /// Retrieve all collections of this database.
    ///
    /// 1. retrieve the names of all collections
    /// 1. cache collections
    ///     - for user collection, construct a `Collection` object and store
    /// them in `self.collections` for later use
    ///     - for system collection, construct a `Collection` object and store
    /// them in `self.system_collections` for later use
    fn fetch_collections(&mut self) -> Result<&mut Database, Error> {
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
                self.collections.insert(coll.name.to_owned(), collection);
            }
        }
        Ok(self)
    }

    pub fn get_url(&self) -> &Url {
        &self.base_url
    }

    pub fn get_session(&self) -> Arc<Client> {
        Arc::clone(&self.session)
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
                info!("System collection {} not found.", name);
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

    pub fn list_collections(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for (name, _) in self.collections.iter() {
            vec.push(name.clone())
        }
        for (name, _) in self.system_collections.iter() {
            vec.push(name.clone())
        }
        vec
    }

    pub fn has_user_collection(&self, name: &str) -> bool {
        match self.get_collection(name) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn list_user_collections(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for (name, _) in self.collections.iter() {
            vec.push(name.clone())
        }
        vec
    }

    pub fn has_system_collection(&self, name: &str) -> bool {
        match self.get_system_collection(name) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn list_system_collections(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for (name, _) in self.system_collections.iter() {
            vec.push(name.clone())
        }
        vec
    }

    pub fn create_edge_collection(&self, name: &str) -> Collection {
        unimplemented!()
    }

    /// Create a collection via HTTP request and add it into `self.collections`.
    ///
    /// Return a database object if success.
    pub fn create_collection(&mut self, name: &str) -> Result<bool, Error> {
        let mut map = HashMap::new();
        map.insert("name", name);
        let url = self.base_url.join("/_api/database").unwrap();
        let resp = self.session.post(url).json(&map).send()?;
        let result: Response<bool> = try_serialize_response(resp);
        match result {
            Response::Ok(resp) => Ok(resp.result),
            Response::Err(error) => Err(format_err!("{}", error.message)),
        }
    }

    /// Drops a collection
    pub fn drop_collection(&self, name: &str) -> Collection {
        unimplemented!()
    }

    /// Execute aql query, return a cursor if succeed. The major advantage of
    /// batch query is that cursors contain more information and stats
    /// about the AQL query, and users can
    /// fetch results in batch to save memory
    /// resources on clients.
    pub fn aql_query_batch<R>(&self, aql: AqlQuery) -> Result<Cursor<R>, Error>
    where
        R: DeserializeOwned + Debug,
    {
        let url = self.base_url.join("cursor").unwrap();
        let resp = self.session.post(url).json(&aql).send()?;
        trace!("{:?}", serde_json::to_string(&aql));
        serialize_query_response(resp)
    }

    /// Get next batch given the cursor id.
    pub fn aql_next_batch<R>(&self, cursor_id: &str) -> Result<Cursor<R>, Error>
    where
        R: DeserializeOwned + Debug,
    {
        let url = self
            .base_url
            .join(&format!("cursor/{}", cursor_id))
            .unwrap();
        let resp = self.session.put(url).send()?;

        serialize_query_response(resp)
    }

    fn aql_fetch_all<R>(&self, response: Cursor<R>) -> Result<Vec<R>, Error>
    where
        R: DeserializeOwned + Debug,
    {
        let mut response_cursor = response;
        let mut results: Vec<R> = Vec::new();
        loop {
            if response_cursor.more {
                let id = response_cursor.id.unwrap().clone();
                results.extend(response_cursor.result.into_iter());
                response_cursor = self.aql_next_batch(id.as_str())?;
            } else {
                break;
            }
        }
        Ok(results)
    }

    /// Execute AQL query fetch all results.
    ///
    /// DO NOT do this when the count of results is too large that network or
    /// memory resources cannot afford.
    ///
    /// DO NOT set a small batch size, otherwise clients will have to make many
    /// HTTP requests.
    pub fn aql_query<R>(&self, aql: AqlQuery) -> Result<Vec<R>, Error>
    where
        R: DeserializeOwned + Debug,
    {
        let response = self.aql_query_batch(aql)?;
        trace!("AQL query response: {:?}", response);
        if response.more {
            self.aql_fetch_all(response)
        } else {
            Ok(response.result)
        }
    }

    /// Similar to `aql_query`, except that this method only accept a string of
    /// AQL query.
    pub fn aql_str<R>(&self, query: &str) -> Result<Vec<R>, Error>
    where
        R: DeserializeOwned + Debug,
    {
        let aql = AqlQuery::new(query);
        self.aql_query(aql)
    }

    /// Similar to `aql_query`, except that this method only accept a string of
    /// AQL query, with additional bind vars.
    pub fn aql_bind_vars<R>(
        &self,
        query: &str,
        bind_vars: Vec<(String, Value)>,
    ) -> Result<Vec<R>, Error>
    where
        R: DeserializeOwned + Debug,
    {
        let mut aql = AqlQuery::new(query);
        for (key, value) in bind_vars {
            aql = aql.bind_var(key, value);
        }
        self.aql_query(aql)
    }
}
