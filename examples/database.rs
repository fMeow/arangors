#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;

use arangors::Connection;

const URL: &str = "http://localhost:8529";

#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
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

#[cfg_attr(feature = "reqwest_blocking")]
fn main() -> Result<(), Error> {
    let conn = Connection::establish_jwt(URL, "username", "password")?;
    let database = conn.db("test_db")?;

    let collections = database.accessible_collections()?;
    println!("{:?}", collections);

    let collections = database.accessible_collections()?;
    println!("{:?}", collections);

    let info = database.info()?;
    println!("{:?}", info);

    Ok(())
}

#[cfg(not(any(
    feature = "reqwest_blocking",
    feature = "reqwest_async",
    feature = "surf_async"
)))]
fn main() {}
