#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;

use arangors::Connection;

const URL: &str = "http://localhost:8529";

// See this example when you want an async code
#[cfg(any(feature = "reqwest_async", feature = "surf_async"))]
#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
async fn main() -> Result<(), Error> {
    let collection_name = "test_collection_create_and_drop";

    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let mut database = conn.db("test_db").await?;
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

// See this example when you want an blocking code
#[cfg(feature = "reqwest_blocking")]
fn main() -> Result<(), Error> {
    let collection_name = "test_collection_create_and_drop";

    let conn = Connection::establish_jwt(URL, "username", "password")?;
    let mut database = conn.db("test_db")?;
    let coll = database.drop_collection(collection_name);
    println!("Should fail: {:?}", coll);

    let coll = database.create_collection(collection_name);
    println!("{:?}", coll);

    let collection = database.collection(collection_name)?;
    println!("id: {:?}", collection.id());
    println!("name: {:?}", collection.name());
    println!("collection_type: {:?}", collection.collection_type());

    let coll = database.drop_collection(collection_name);
    println!("{:?}", coll);

    Ok(())
}

#[cfg(not(any(
    feature = "reqwest_blocking",
    feature = "reqwest_async",
    feature = "surf_async"
)))]
fn main() {}
