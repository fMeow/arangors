//! Top level connection object that hold a http client (either synchronous or
//! asynchronous), arango URL, and buffered accessible databases object.
//!
//! ## Establishing connections
//! There is three way to establish connections:
//! - jwt
//! - basic auth
//! - no authentication
//!
//! So are the `arangors` API:
//! Example:
//!
//! ```rust,ignore
//! use arangors::{connection::auth::Auth, Connection};
//!
//! // Basic auth
//! let auth = Auth::basic(username.into(), password.into());
//! let conn = Connection::establish("http://localhost:8529", auth).unwrap();
//!
//! // JWT Auth
//! let auth = Auth::jwt(username.into(), password.into());
//! let conn = Connection::establish("http://localhost:8529", auth).unwrap();
//!
//! // Without Auth
//! let conn = Connection::establish("http://localhost:8529", Auth::None).unwrap();
//!
//! // (Recommended) Handy functions
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
//! let conn =
//!     Connection::establish_basic_auth("http://localhost:8529", "username", "password").unwrap();
//! let conn = Connection::establish_without_auth("http://localhost:8529").unwrap();
//! ```

mod auth;
mod model;
#[cfg(test)]
mod tests;

use failure::{format_err, Error};
use log::{error, info, trace};
use std::{collections::HashMap, sync::Arc};

// use reqwest::unstable::r#async::Client;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, SERVER},
    Client, Url,
};
use serde_derive::Deserialize;

use self::auth::Auth;
use self::model::{DatabaseInfo, Version};
use super::database::Database;
use super::response::{serialize_response, try_serialize_response, Response};

/// Connection is the top level API for this crate.
/// It contains a http client, information about auth, arangodb url, and a hash
/// map of the databases Object. The `databases` Hashmap is construct once
/// connections succeed.
/// ## Initialization
/// There is two way to initialize `Connection`
/// - Default value
/// ```rust
/// use arangors::connection::Connection;
/// let conn: Connection = Default::default();
/// ```
// TODO Connections' lifetimes should be longer than Databases' lifetimes
#[derive(Debug)]
pub struct Connection {
    session: Arc<Client>,
    databases: HashMap<String, Database>,
    arango_url: Url,
}

impl Connection {
    /// Validate the server at given arango url
    ///
    /// Cast `failure::Error` if
    /// - Connection failed
    /// - response code is not 200
    /// - no SERVER header in response header
    /// - validate_server in response header is not `ArangoDB`
    pub fn validate_server(&self) -> Result<(), Error> {
        validate_server(self.arango_url.as_str())
    }

    fn jwt_login<S: Into<String>>(&self, username: S, password: S) -> Result<String, Error> {
        #[derive(Deserialize)]
        struct JWT {
            pub jwt: String,
        }
        let url = self.arango_url.join("/_open/auth").unwrap();

        let mut map = HashMap::new();
        map.insert("username", username.into());
        map.insert("password", password.into());

        let jwt: JWT = self.session.post(url).json(&map).send()?.json()?;
        Ok(jwt.jwt)
    }

    // -------------------- methods for initialization --------------------
    /// The most trivial way to establish a connection to arangoDB server.
    /// Users have to build a `Auth` object themselves.
    /// The recommended way to establish connection is to use the
    /// functions that specify the authentication methods:
    /// - establish_without_auth
    /// - establish_basic_auth
    /// - establish_jwt
    ///
    /// The most secure way to connect to a arangoDB server is via JWT
    /// token authentication, along with TLS encryption.
    ///
    /// The connection is establish in the following steps:
    /// 1. validate if it is a arangoDB server at the given base url
    /// 1. set authentication in header
    /// 1. build a http client that holds authentication tokens
    /// 1. construct databases objects for later use
    pub fn establish<S: Into<String>>(arango_url: S, auth: Auth) -> Result<Connection, Error> {
        let mut conn = Connection {
            arango_url: Url::parse(arango_url.into().as_str())?.join("/").unwrap(),
            ..Default::default()
        };
        conn.validate_server()?;

        let authorization = match auth {
            Auth::Basic(credential) => {
                let token =
                    base64::encode(&format!("{}:{}", credential.username, credential.password));
                Some(format!("Basic {}", token))
            }
            Auth::Jwt(credential) => {
                let token = conn.jwt_login(credential.username, credential.password)?;
                Some(format!("Bearer {}", token))
            }
            Auth::None => None,
        };

        let mut headers = HeaderMap::new();
        if let Some(value) = authorization {
            headers.insert(AUTHORIZATION, value.parse().unwrap());
        }

        conn.session = Arc::new(
            Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?,
        );
        conn.fetch_databases()?;
        info!("Established");
        Ok(conn)
    }

    pub fn establish_without_auth<S: Into<String>>(arango_url: S) -> Result<Connection, Error> {
        trace!("Establish without auth");
        Ok(Connection::establish(arango_url.into(), Auth::None)?)
    }

    pub fn establish_basic_auth<S: Into<String>>(
        arango_url: S,
        username: S,
        password: S,
    ) -> Result<Connection, Error> {
        trace!("Establish with basic auth");
        Ok(Connection::establish(
            arango_url.into(),
            Auth::basic(username.into(), password.into()),
        )?)
    }
    pub fn establish_jwt<S: Into<String>>(
        arango_url: S,
        username: S,
        password: S,
    ) -> Result<Connection, Error> {
        trace!("Establish with jwt");
        Ok(Connection::establish(
            arango_url.into(),
            Auth::jwt(username.into(), password.into()),
        )?)
    }

    pub fn get_url(&self) -> &Url {
        &self.arango_url
    }

    pub fn get_session(&self) -> Arc<Client> {
        Arc::clone(&self.session)
    }

    /// Get database object with name.
    ///
    /// This function look up accessible database in cache hash map,
    /// and return a reference of database if found.
    pub fn db(&self, name: &str) -> Option<&Database> {
        match self.databases.get(name) {
            Some(database) => Some(&database),
            None => {
                info!("Database {} not found.", name);
                None
            }
        }
    }

    /// Get a hashmap of name-reference for all database.
    pub fn get_all_db(&self) -> HashMap<String, &Database> {
        let mut databases: HashMap<String, &Database> = HashMap::new();
        for (name, database) in self.databases.iter() {
            databases.insert(name.to_owned(), database);
        }
        databases
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
    fn fetch_databases(&mut self) -> Result<&mut Connection, Error> {
        // an invalid arango_url should never running through initialization
        // so we assume arango_url is a valid url
        // When we pass an invalid path, it should panic to eliminate the bug
        // in development.
        let url = self.arango_url.join("/_api/database/user").unwrap();
        let resp = self.session.get(url).send()?;
        let result: Vec<String> = serialize_response(resp)?;
        trace!("Retrieved databases.");
        for database_name in result.iter() {
            self.databases.insert(
                database_name.to_owned(),
                Database::new(&self, database_name.as_str())?,
            );
        }
        Ok(self)
    }

    pub fn fetch_arango_version(&self) -> Result<Version, Error> {
        let url = self.arango_url.join("/_api/version").unwrap();
        let version: Version = self.session.get(url).send()?.json()?;
        Ok(version)
    }

    /// List all existing databases in server. As clients may not has the
    /// permission to access all the databases, this function only return
    /// a `Vec<String>` instead of a hash map of databases.
    pub fn list_all_database(&self) -> Result<Vec<&String>, Error> {
        let mut vec: Vec<&String> = Vec::new();

        for key in self.databases.keys() {
            vec.push(key);
        }

        Ok(vec)
    }

    /// Get a pointer of current database.
    /// Note that this function would make a request to arango server.
    ///
    /// Personally speaking, I don't know why we need to know the current
    /// database. As we never need to know the database as long as we get
    /// the id of collections.
    pub fn fetch_current_database(&self) -> Result<DatabaseInfo, Error> {
        let url = self.arango_url.join("/_api/database/current").unwrap();
        let resp = self.session.get(url).send()?;
        let result: DatabaseInfo = serialize_response(resp)?;
        Ok(result)
    }

    /// Create a database via HTTP request and add it into `self.databases`.
    ///
    /// If creation fails, an Error is cast. Otherwise, a bool is returned to
    /// indicate whether the database is correctly created.
    pub fn create_database(&mut self, name: &str) -> Result<bool, Error> {
        let mut map = HashMap::new();
        map.insert("name", name);
        let url = self.arango_url.join("/_api/database").unwrap();
        let resp = self.session.post(url).json(&map).send()?;
        let result: Response<bool> = try_serialize_response(resp);
        match result {
            Response::Ok(resp) => {
                self.databases
                    .insert(name.to_owned(), Database::new(&self, name)?);
                Ok(resp.result)
            }
            Response::Err(error) => Err(format_err!("{}", error.message)),
        }
    }

    /// Drop database with name.
    ///
    /// If the database is successfully dropped, return the dropped database.
    /// The ownership of the dropped database would be moved out. And the
    /// dropped database can no longer be found at `self.databases`.
    pub fn drop_database(&self, name: &str) -> Result<bool, Error> {
        let url_path = format!("/_api/database/{}", name);
        let url = self.arango_url.join(&url_path).unwrap();
        let resp = self.session.delete(url).send()?;
        let result: Response<bool> = try_serialize_response(resp);
        match result {
            Response::Ok(resp) => Ok(resp.result),
            Response::Err(error) => Err(format_err!("{}", error.message)),
        }
    }

    /// Refresh the hierarchy of all accessible databases.
    ///
    /// This is a expensive method, and all the cached information about
    /// this server would be refreshed.
    ///
    /// Refresh is done in the following steps:
    /// 1. retrieve the names of all the accessible databases
    /// 1. for each databases, construct a `Database` object and store them in
    /// `self.databases` for later use
    ///
    /// Note that a `Database` object caches all the accessible collections.
    ///
    /// This function uses the API that is used to retrieve a list of
    /// all databases the current user can access to refresh databases.
    /// Then each database retrieve a list of available collections.
    pub fn refresh(&mut self) -> Result<&mut Connection, Error> {
        self.fetch_databases()
    }
}

impl Default for Connection {
    fn default() -> Connection {
        Connection {
            arango_url: Url::parse("http://127.0.0.1:8529").unwrap(),
            databases: HashMap::new(),
            session: Arc::new(Client::new()),
        }
    }
}

/// Validate the server at given arango url
/// return false if
/// - Connection failed
/// - response code is not 200
/// - no SERVER header in response header
/// - SERVER header in response header is not `ArangoDB`
pub fn validate_server<'b>(arango_url: &'b str) -> Result<(), Error> {
    let mut result = false;
    let resp = reqwest::get(arango_url)?;
    // HTTP code 200
    if resp.status().is_success() {
        // have `Server` in header
        if let Some(server) = resp.headers().get(SERVER) {
            // value of `Server` is `ArangoDB`
            let server_value = server.to_str().unwrap();
            if server_value.eq_ignore_ascii_case("ArangoDB") {
                result = true;
                info!("Validate arangoDB server done.");
            } else {
                error!("In HTTP header, Server is {}", server_value);
            }
        } else {
            error!("Fail to find Server in HTTP header");
        }
    }
    if result == true {
        Ok(())
    } else {
        Err(format_err!("Cannot find valid ArangoDB server"))
    }
}
