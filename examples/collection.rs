#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;

use arangors::Connection;

const URL: &str = "http://localhost:8529";

#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
#[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
async fn main() -> Result<(), Error> {
    let collection_name = "test_collection_create_and_drop";

    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let database = conn.db("test_db").await?;
    let coll = database.drop_collection(collection_name).await;
    println!("Should fail: {:?}", coll);

    let coll = database.create_collection(collection_name).await;
    println!("{:?}", coll);

    let collection = database.collection(collection_name).await?;
    println!("id: {:?}", collection.id());
    println!("name: {:?}", collection.name());
    println!("collection_type: {:?}", collection.collection_type());

    let coll = database.drop_collection(collection_name).await;
    println!("{:?}", coll);

    Ok(())
}
