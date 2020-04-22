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
/// ```rust, ignore
/// use arangors::connection::Auth;
///
/// let basic_auth = Auth::basic("username", "password");
/// let jwt_auth = Auth::jwt("username", "password");
/// let no_auth = Auth::None;
/// let no_auth = Auth::default();
/// ```
#[derive(Debug, Clone)]
pub(crate) enum Auth<'a> {
    /// Basic auth
    Basic(Credential<'a>),
    /// JSON Web Token (JWT) auth
    Jwt(Credential<'a>),
    /// no auth
    None,
}

impl<'a> Default for Auth<'a> {
    fn default() -> Auth<'static> {
        Auth::None
    }
}

impl<'a> Auth<'a> {
    pub fn basic(username: &'a str, password: &'a str) -> Auth<'a> {
        Auth::Basic(Credential { username, password })
    }

    pub fn jwt(username: &'a str, password: &'a str) -> Auth<'a> {
        Auth::Jwt(Credential { username, password })
    }
}

/// Username and password holder for authentication
#[derive(Debug, Clone, Hash)]
pub(crate) struct Credential<'a> {
    /// username
    pub username: &'a str,
    /// password
    pub password: &'a str,
}
