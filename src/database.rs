use failure::Error;
use std::rc::Rc;

use reqwest::{Client, Url};

use super::connection::Connection;

#[derive(Debug, Clone)]
pub struct Database {
    name: String,
    base_url: Url,
    connection: Rc<Client>,
}
impl<'a, 'b: 'a> Database {
    pub fn new<T: Into<String>>(conn: &'b Connection, name: T) -> Result<Database, Error> {
        let name = name.into();
        let path = format!("/_db/{}/_api", name.as_str());
        let url = Url::parse(conn.get_url().as_str())?.join(path.as_str())?;
        Ok(Database {
            name,
            connection: conn.get_session(),
            base_url: url,
        })
    }
}
