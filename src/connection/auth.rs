//! Type definitions for various authentication methods.

/// According to aragndb document, supported auth methods are
/// - basicAuth
/// - JWT
/// - no auth
///
/// And this enum provides an abstraction to these methods.
///
/// Auth is then used when initialize `Connection`.
///
/// # Example
/// ```rust,ignore
/// use arango_rs::connection::auth::Auth;
/// let basic_auth = Auth::basic("user","123456");
/// let jwt_auth = Auth::jwt("user","123456");
///
/// let no_auth = Auth::None;
/// let no_auth = Auth::default();
/// ```
#[derive(Debug, Clone)]
pub enum Auth {
    /// Basic auth
    Basic(Credential),
    /// JSON Web Token (JWT) auth
    Jwt(Credential),
    /// no auth
    None,
}

impl Default for Auth {
    fn default() -> Auth {
        Auth::None
    }
}

impl Auth {
    pub fn basic<T: Into<String>>(username: T, password: T) -> Auth {
        Auth::Basic(Credential {
            username: username.into(),
            password: password.into(),
        })
    }

    pub fn jwt<T: Into<String>>(username: T, password: T) -> Auth {
        Auth::Jwt(Credential {
            username: username.into(),
            password: password.into(),
        })
    }
}

/// Username and password holder for authentication
#[derive(Debug, Clone, Hash)]
pub struct Credential {
    /// username
    pub username: String,
    /// password
    pub password: String,
}
