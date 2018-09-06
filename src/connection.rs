//! Top level connection object that hold a http client (either synchronous or
//! asynchronous), user authentication information, and buffers databases object.
//!
//!

use failure::{format_err, Error};
use std::collections::HashMap;

// use reqwest::r#async::Client;
use reqwest::{
    header::{Authorization, Basic, Bearer, Headers, Server},
    Client, Url,
};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;

mod auth;
#[cfg(test)]
mod tests;
use self::auth::Auth;
use super::database::Database;
use super::result::Response;

/// Connection is the top level API for this crate.
/// It contains a http client, information about auth, arangodb url, and a hash map
/// of the databases Object. The `databases` Hashmap is construct once connections
/// succeed.
/// ## Initialization
/// There is two way to initialize `Connection`
/// - Default value
/// ```rust
/// use arango_rs::connection::Connection;
/// let conn: Connection = Default::default();
/// ```
///
// TODO Connections' lifetimes should be longer than Databases' lifetimes
#[derive(Debug)]
pub struct Connection<'a> {
    session: Rc<Client>,
    databases: HashMap<String, Database<'a>>,
    arango_url: Url,
}

impl<'a> Connection<'a> {
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
    pub fn establish<S: Into<String>>(arango_url: S, auth: Auth) -> Result<Connection<'a>, Error> {
        let mut conn = Connection {
            arango_url: Url::parse(arango_url.into().as_str())?,
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
        conn.session = Client::builder()
            .gzip(true)
            .default_headers(headers)
            .build()?;
        conn.retrieve_databases()?;
        Ok(Connection {
            ..Default::default()
        })
    }

    pub fn establish_without_auth<S: Into<String>>(arango_url: S) -> Result<Connection<'a>, Error> {
        Ok(Connection::establish(arango_url.into(), Auth::None)?)
    }

    pub fn establish_basic_auth<S: Into<String>>(
        arango_url: S,
        username: S,
        password: S,
    ) -> Result<Connection<'a>, Error> {
        Ok(Connection::establish(
            arango_url.into(),
            Auth::basic(username.into(), password.into()),
        )?)
    }
    pub fn establish_jwt<S: Into<String>>(
        arango_url: S,
        username: S,
        password: S,
    ) -> Result<Connection<'a>, Error> {
        Ok(Connection::establish(
            arango_url.into(),
            Auth::jwt(username.into(), password.into()),
        )?)
    }

    pub fn get_url(&'a self) -> &'a Url {
        &self.arango_url
    }

    pub fn get_session(&'a self) -> &'a Client {
        &self.session
    }

    /// There are different type of json object when requests to arangoDB
    /// server is accepted or not. Here provides an abstraction for
    /// response of success and failure.
    /// TODO more intuitive response error enum
    fn serialize_response<T>(mut resp: reqwest::Response) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response: Response<T> = resp.json()?;
        match response {
            Response::Success { result, .. } => Ok(result),
            Response::Error { message, .. } => Err(format_err!("{}", message)),
        }
    }

    /// The last steps of connection establishment is to query the accessible
    /// databases and cache them in a hashmap of `Databases` objects.
    ///
    /// 1. retrieve the names of all the accessible databases
    /// 1. for each databases, construct a `Database` object and store them in
    /// `self.databases` for later use
    fn retrieve_databases<'b>(&'a mut self) -> Result<&mut Connection, Error> {
        let url = self.arango_url.join("/_api/database/user")?;
        let resp = self.session.get(url).send()?;
        let result: Vec<String> = Connection::serialize_response(resp)?;
        for database_name in result.iter() {
            self.databases.insert(
                database_name.to_owned(),
                Database::new(&self, database_name.as_str())?,
            );
        }
        Ok(self)
    }
}
impl<'a> Default for Connection<'a> {
    fn default() -> Connection<'a> {
        Connection {
            arango_url: Url::parse("http://127.0.0.1:8529").unwrap(),
            databases: HashMap::new(),
            session: Client::new(),
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
            }
        }
    }
    if result == true {
        Ok(())
    } else {
        Err(format_err!("Cannot find valid ArangoDB server"))
    }
}
