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
//! - With authentication
//!
//! ```rust
//! use arangors::Connection;
//!
//! let conn = Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
//! let conn =
//!     Connection::establish_basic_auth("http://localhost:8529", "username", "password").unwrap();
//! ```
//!
//! - No authentication
//! ```rust, ignore
//! use arangors::Connection;
//! let conn = Connection::establish_without_auth("http://localhost:8529").unwrap();
//! ```

use std::{collections::HashMap, sync::Arc};

use failure::{format_err, Error};
use log::{error, info, trace};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, SERVER},
    Client, Url,
};
use serde::de::value::StrDeserializer;
use serde::{Deserialize, Serialize};

use super::database::Database;
use super::response::{serialize_response, try_serialize_response, Response};

use self::auth::Auth;
use self::model::{DatabaseInfo, Version};
use self::role::{Admin, Normal};

mod auth;
pub mod model;
#[cfg(test)]
mod tests;

pub mod role {
    #[derive(Debug)]
    pub struct Normal;

    #[derive(Debug)]
    pub struct Admin;
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Permission {
    #[serde(rename = "none")]
    NoAccess,
    #[serde(rename = "ro")]
    ReadOnly,
    #[serde(rename = "rw")]
    ReadWrite,
}

/// Connection is the top level API for this crate.
/// It contains a http client, information about auth, arangodb url, and a hash
/// map of the databases Object. The `databases` Hashmap is construct once
/// connections succeed.
#[derive(Debug)]
pub struct Connection<S> {
    session: Arc<Client>,
    arango_url: Url,
    username: String,
    state: S,
    pub(crate) phantom: (),
}

impl<S> Connection<S> {
    /// Validate the server at given arango url
    ///
    /// Cast `failure::Error` if
    /// - Connection failed
    /// - response code is not 200
    /// - no SERVER header in response header
    /// - SERVER header in response header is not `ArangoDB`
    pub fn validate_server(&self) -> Result<(), Error> {
        let arango_url = self.arango_url.as_str();
        let resp = reqwest::get(arango_url)?;
        // HTTP code 200
        if resp.status().is_success() {
            // have `Server` in header
            if let Some(server) = resp.headers().get(SERVER) {
                // value of `Server` is `ArangoDB`
                let server_value = server.to_str().unwrap();
                if server_value.eq_ignore_ascii_case("ArangoDB") {
                    trace!("Validate arangoDB server done.");
                    return Ok(());
                } else {
                    return Err(format_err!("In HTTP header, Server is {}", server_value));
                }
            } else {
                return Err(format_err!("Fail to find Server in HTTP header"));
            }
        } else {
            return Err(format_err!(
                "Fail to connect to server, Status code: {}",
                resp.status()
            ));
        }
    }

    /// Get url for remote arangoDB server.
    pub fn get_url(&self) -> &Url {
        &self.arango_url
    }

    /// Get HTTP session.
    ///
    /// Users can use this method to get a authorized session to access
    /// arbitrary path on arangoDB Server.
    ///
    /// TODO This method should only be public in this crate when all features
    ///     are implemented.
    pub fn get_session(&self) -> Arc<Client> {
        Arc::clone(&self.session)
    }

    /// Get database object with name.
    ///
    /// This function look up accessible database in cache hash map,
    /// and return a reference of database if found.
    pub fn db(&self, name: &str) -> Result<Database, Error> {
        let dbs = self.accessible_databases()?;
        if dbs.contains_key(name) {
            Ok(Database::new(&self, name))
        } else {
            Err(format_err!("Cannot access to db: {}", name))
        }
    }

    /// Get a list of accessible database
    /// 1. retrieve the names of all the accessible databases
    /// 1. for each databases, construct a `Database` object and store them in
    /// `self.databases` for later use
    ///
    /// This function uses the API that is used to retrieve a list of
    /// all databases the current user can access.
    pub fn accessible_databases(&self) -> Result<HashMap<String, Permission>, Error> {
        let url = self
            .arango_url
            .join(&format!("/_api/user/{}/database", &self.username))
            .unwrap();
        let resp = self.session.get(url).send()?;
        let result = serialize_response(resp)?;
        Ok(result)
    }
}

impl Connection<Normal> {
    /// Establish connection to ArangoDB sever with Auth.
    ///
    /// The connection is establish in the following steps:
    /// 1. validate if it is a arangoDB server at the given base url
    /// 1. set authentication in header
    /// 1. build a http client that holds authentication tokens
    /// 1. construct databases objects for later use
    ///
    /// The most secure way to connect to a arangoDB server is via JWT
    /// token authentication, along with TLS encryption.
    fn establish<T: Into<String>>(arango_url: T, auth: Auth) -> Result<Connection<Normal>, Error> {
        let mut conn = Connection {
            arango_url: Url::parse(arango_url.into().as_str())?.join("/").unwrap(),
            username: String::new(),
            session: Arc::new(Client::new()),
            state: Normal,
            phantom: (),
        };
        conn.validate_server()?;

        let mut user: String;
        let authorization = match auth {
            Auth::Basic(cred) => {
                user = String::from(cred.username);

                let token = base64::encode(&format!("{}:{}", cred.username, cred.password));
                Some(format!("Basic {}", token))
            }
            Auth::Jwt(cred) => {
                user = String::from(cred.username);

                let token = conn.jwt_login(cred.username, cred.password)?;
                Some(format!("Bearer {}", token))
            }
            Auth::None => {
                user = String::from("root");
                None
            }
        };

        let mut headers = HeaderMap::new();
        if let Some(value) = authorization {
            headers.insert(AUTHORIZATION, value.parse().unwrap());
        }

        conn.username = user;
        conn.session = Arc::new(
            Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?,
        );
        info!("Established");
        Ok(conn)
    }

    /// Establish connection to ArangoDB sever without Authentication.
    ///
    /// The target server **MUST DISABLE** authentication for all requests,
    /// which should only used for **test purpose**.
    ///
    /// Disable authentication means all operations are performed by root user.
    ///
    /// Example:
    /// ```rust, ignore
    /// use arangors::Connection;
    ///
    /// let conn = Connection::establish_without_auth("http://localhost:8529").unwrap();
    /// ```
    pub fn establish_without_auth<T: Into<String>>(
        arango_url: T,
    ) -> Result<Connection<Normal>, Error> {
        trace!("Establish without auth");
        Ok(Connection::establish(arango_url.into(), Auth::None)?)
    }

    /// Establish connection to ArangoDB sever with basic auth.
    ///
    /// Example:
    /// ```rust
    /// use arangors::Connection;
    ///
    /// let conn =
    ///     Connection::establish_basic_auth("http://localhost:8529", "username", "password").unwrap();
    /// ```
    pub fn establish_basic_auth(
        arango_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Connection<Normal>, Error> {
        trace!("Establish with basic auth");
        Ok(Connection::establish(
            arango_url,
            Auth::basic(username, password),
        )?)
    }

    /// Establish connection to ArangoDB sever with jwt authentication.
    ///
    /// Prefered way to interact with arangoDB server.
    ///
    /// JWT token expires after 1 month.
    ///
    /// Example:
    ///
    /// ```rust
    /// use arangors::Connection;
    ///
    /// let conn = Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
    /// ```
    pub fn establish_jwt(
        arango_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Connection<Normal>, Error> {
        trace!("Establish with jwt");
        Ok(Connection::establish(
            arango_url,
            Auth::jwt(username, password),
        )?)
    }

    fn jwt_login<T: Into<String>>(&self, username: T, password: T) -> Result<String, Error> {
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

    pub fn into_admin(self) -> Result<Connection<Admin>, Error> {
        let dbs = self.accessible_databases()?;
        let db = dbs
            .get("_system")
            .ok_or(format_err!("Do not have read access to _system database"))?;
        match db {
            Permission::ReadWrite => Ok(self.into()),
            _ => Err(format_err!("Do not have write access to _system database")),
        }
    }
}

impl Connection<Admin> {
    /// Create a database via HTTP request and add it into `self.databases`.
    ///
    /// If creation fails, an Error is cast. Otherwise, a bool is returned to
    /// indicate whether the database is correctly created.
    ///
    /// TODO tweak options on creating database
    pub fn create_database(&self, name: &str) -> Result<Database, Error> {
        let mut map = HashMap::new();
        map.insert("name", name);
        let url = self.arango_url.join("/_api/database").unwrap();
        let resp = self.session.post(url).json(&map).send()?;
        let result: Response<bool> = try_serialize_response(resp);
        match result {
            Response::Ok(resp) => {
                if resp.result == true {
                    Ok(self.db(name)?)
                } else {
                    Err(format_err!("Fail to create db. Reason: {:?}", resp))
                }
            }
            Response::Err(error) => Err(format_err!("{}", error.message)),
        }
    }

    /// Drop database with name.
    pub fn drop_database(&mut self, name: &str) -> Result<(), Error> {
        let url_path = format!("/_api/database/{}", name);
        let url = self.arango_url.join(&url_path).unwrap();
        let resp = self.session.delete(url).send()?;
        let result: Response<bool> = try_serialize_response(resp);
        match result {
            Response::Ok(resp) => {
                if resp.result == true {
                    Ok(())
                } else {
                    Err(format_err!("Fail to drop db. Reason: {:?}", resp))
                }
            }
            Response::Err(error) => Err(format_err!("{}", error.message)),
        }
    }

    pub fn into_normal(self) -> Connection<Normal> {
        self.into()
    }
}

impl From<Connection<Normal>> for Connection<Admin> {
    fn from(conn: Connection<Normal>) -> Connection<Admin> {
        Connection {
            arango_url: conn.arango_url,
            session: conn.session,
            username: conn.username,
            state: Admin,
            phantom: (),
        }
    }
}

impl From<Connection<Admin>> for Connection<Normal> {
    fn from(conn: Connection<Admin>) -> Connection<Normal> {
        Connection {
            arango_url: conn.arango_url,
            session: conn.session,
            username: conn.username,
            state: Normal,
            phantom: (),
        }
    }
}
