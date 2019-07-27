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
// use reqwest::unstable::r#async::Client;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, SERVER},
    Client, Url,
};
use serde::de::value::StrDeserializer;
use serde_derive::Deserialize;

use super::database::Database;
use super::response::{serialize_response, try_serialize_response, Response};

use self::auth::Auth;
use self::model::{DatabaseInfo, Version};

mod auth;
pub mod model;
#[cfg(test)]
mod tests;

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
#[derive(Debug)]
pub struct Connection {
    session: Arc<Client>,
    arango_url: Url,
    username: String,
    //    role: R,
}

impl Connection {
    /// Validate the server at given arango url
    ///
    /// Cast `failure::Error` if
    /// - Connection failed
    /// - response code is not 200
    /// - no SERVER header in response header
    /// - SERVER header in response header is not `ArangoDB`
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
    fn establish<S: Into<String>>(arango_url: S, auth: Auth) -> Result<Connection, Error> {
        let mut conn = Connection {
            arango_url: Url::parse(arango_url.into().as_str())?.join("/").unwrap(),
            ..Default::default()
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
    pub fn establish_without_auth<S: Into<String>>(arango_url: S) -> Result<Connection, Error> {
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
    ) -> Result<Connection, Error> {
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
    ) -> Result<Connection, Error> {
        trace!("Establish with jwt");
        Ok(Connection::establish(
            arango_url,
            Auth::jwt(username, password),
        )?)
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
    pub fn db(&self, name: &str) -> Option<&Database> {
        unimplemented!()
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
    fn databases(&mut self) -> Result<Database, Error> {
        let url = self
            .arango_url
            .join(&format!("/_api/user/{}/database", &self.username))
            .unwrap();
        let resp = self.session.get(url).send()?;
        let result: Vec<String> = serialize_response(resp)?;
        //        Ok()
        unimplemented!()
    }

    pub fn fetch_arango_version(&self) -> Result<Version, Error> {
        let url = self.arango_url.join("/_api/version").unwrap();
        let version: Version = self.session.get(url).send()?.json()?;
        Ok(version)
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
    ///
    /// TODO tweak options on ceating database
    pub fn create_database(&mut self, name: &str) -> Result<bool, Error> {
        //        let mut map = HashMap::new();
        //        map.insert("name", name);
        //        let url = self.arango_url.join("/_api/database").unwrap();
        //        let resp = self.session.post(url).json(&map).send()?;
        //        let result: Response<bool> = try_serialize_response(resp);
        //        match result {
        //            Response::Ok(resp) => {
        //                self.databases
        //                    .insert(name.to_owned(), Database::new(&self, name)?);
        //                Ok(resp.result)
        //            }
        //            Response::Err(error) => Err(format_err!("{}", error.message)),
        //        }
        unimplemented!();
    }

    /// Drop database with name.
    pub fn drop_database(&self, name: &str) -> Result<bool, Error> {
        //        let url_path = format!("/_api/database/{}", name);
        //        let url = self.arango_url.join(&url_path).unwrap();
        //        let resp = self.session.delete(url).send()?;
        //        let result: Response<bool> = try_serialize_response(resp);
        //        match result {
        //            Response::Ok(resp) => Ok(resp.result),
        //            Response::Err(error) => Err(format_err!("{}", error.message)),
        //        }
        unimplemented!()
    }

    //    /// Refresh the hierarchy of all accessible databases.
    //    ///
    //    /// This is a expensive method, and all the cached information about
    //    /// this server would be refreshed.
    //    ///
    //    /// Refresh is done in the following steps:
    //    /// 1. retrieve the names of all the accessible databases
    //    /// 1. for each databases, construct a `Database` object and store them in
    //    /// `self.databases` for later use
    //    ///
    //    /// Note that a `Database` object caches all the accessible collections.
    //    ///
    //    /// This function uses the API that is used to retrieve a list of
    //    /// all databases the current user can access to refresh databases.
    //    /// Then each database retrieve a list of available collections.
    //    pub fn refresh(&mut self) -> Result<&mut Connection, Error> {
    //        self.fetch_databases()
    //    }
}

impl Default for Connection {
    fn default() -> Connection {
        Connection {
            arango_url: Url::parse("http://127.0.0.1:8529").unwrap(),
            username: String::from("root"),
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
