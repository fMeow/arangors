#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;

use arangors::Connection;

const URL: &str = "http://localhost:8529";

#[cfg_attr(not(feature = "blocking"), tokio::main)]
#[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
async fn main() -> Result<(), Error> {
    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let database = conn.db("test_db").await?;

    let collections = database.accessible_collections().await?;
    println!("{:?}", collections);

    let collections = database.accessible_collections().await?;
    println!("{:?}", collections);

    let info = database.info().await?;
    println!("{:?}", info);

    Ok(())
}
