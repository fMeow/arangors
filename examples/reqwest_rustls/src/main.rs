//! An example to use reqwest with rustls.
//!
//! Just
use arangors::Connection;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    const URL: &str = "http://localhost:8529";
    // use custom Reqwest Client
    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let db = conn.db("test_db").await?;
    let info = db.info().await?;
    println!("{:?}", info);

    Ok(())
}
