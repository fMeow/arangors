//! Top level connection object that hold a http client (either synchronous or
//! asynchronous), arango URL, and buffered accessible databases object.
//!
//! For now, the http client is **synchronous** only.
//!

use failure::{format_err, Error};
use log::{error, info, trace};
use std::{collections::HashMap, rc::Rc};

// use reqwest::r#async::Client;
use reqwest::{
    header::{Authorization, Basic, Bearer, Headers, Server},
    Client, Url,
};
use serde_derive::Deserialize;

mod auth;
#[cfg(test)]
mod tests;
use self::auth::Auth;
use super::database::Database;
use super::response::{get_result, Response};

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
    session: Rc<Client>,
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
        let url = self.arango_url.join("/_open/auth")?;

        let mut map = HashMap::new();
        map.insert("username", username.into());
        map.insert("password", password.into());

        let client = reqwest::Client::new();
        let jwt: JWT = client.post(url).json(&map).send()?.json()?;
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
            arango_url: Url::parse(arango_url.into().as_str())?.join("/")?,
            ..Default::default()
        };
        conn.validate_server()?;
        let mut headers = Headers::new();
        match auth {
            Auth::Basic(credential) => headers.set(Authorization(Basic {
                username: credential.username.to_owned(),
                password: Some(credential.password.to_owned()),
            })),
            Auth::Jwt(credential) => {
                let token = conn.jwt_login(credential.username, credential.password)?;
                headers.set(Authorization(Bearer {
                    token: token.to_owned(),
                }))
            }
            Auth::None => {}
        };
        conn.session = Rc::new(
            Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()?,
        );
        conn.retrieve_databases()?;
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

    pub fn get_session(&self) -> Rc<Client> {
        Rc::clone(&self.session)
    }

    /// Get database object with name.
    ///
    /// This function look up accessible database in cache hash map,
    /// and return a reference of database if found.
    pub fn get_database(&self, name: &str) -> Option<&Database> {
        match self.databases.get(name) {
            Some(database) => Some(&database),
            None => {
                info!("Database {} not found.", name);
                None
            }
        }
    }

    /// Get a hashmap of name-reference for all database.
    pub fn get_all_database(&self) -> HashMap<String, &Database> {
        let databases: HashMap<String, &Database> = HashMap::new();
        self.databases
            .iter()
            .map(|(name, database)| databases.insert(name.to_owned(), &database));
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
    fn retrieve_databases(&mut self) -> Result<&mut Connection, Error> {
        // an invalid arango_url should never running through initialization
        // so we assume arango_url is a valid url
        // When we pass an invalid path, it should panic to eliminate the bug
        // in development.
        let url = self.arango_url.join("/_api/database/user").unwrap();
        let resp = self.session.get(url).send()?;
        let result: Vec<String> = get_result(resp)?;
        trace!("Retrieved databases.");
        for database_name in result.iter() {
            self.databases.insert(
                database_name.to_owned(),
                Database::new(&self, database_name.as_str())?,
            );
        }
        Ok(self)
    }

    pub fn retrieve_arango_version(&self) -> &str {
        unimplemented!();
    }

    /// Create a database via HTTP request and add it into `self.databases`.
    ///
    /// Return a database object if success.
    pub fn create_database(&self) -> Result<&Database, Error> {
        unimplemented!();
    }

    /// List all existing databases in server. As clients may not has the
    /// permission to access all the databases, this function only return
    /// a `Vec<String>` instead of a hash map of databases.
    pub fn list_all_database(&self) -> Result<Vec<String>, Error> {
        unimplemented!();
    }

    /// Get a pointer of current database.
    /// Note that this function would make a request to arango server.
    ///
    /// Personally speaking, I don't know why we need to know the current
    /// database. As we never need to know the database as long as we get
    /// the id of collections.
    pub fn current_database(&self) -> Result<&Database, Error> {
        unimplemented!()
    }

    /// Drop database with name.
    ///
    /// If the database is successfully dropped, return the dropped database.
    /// The ownership of the dropped database would be moved out. And the
    /// dropped database can no longer be found at `self.databases`.
    pub fn drop_database<T: Into<String>>(&self, name: T) -> Result<Database, Error> {
        unimplemented!()
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
        self.retrieve_databases()
    }
}
impl Default for Connection {
    fn default() -> Connection {
        Connection {
            arango_url: Url::parse("http://127.0.0.1:8529").unwrap(),
            databases: HashMap::new(),
            session: Rc::new(Client::new()),
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
        if let Some(server) = resp.headers().get::<Server>() {
            // value of `Server` is `ArangoDB`
            if server.eq_ignore_ascii_case("ArangoDB") {
                result = true;
                info!("Validate arangoDB server done.");
            } else {
                error!("In HTTP header, Server is {}", server);
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
