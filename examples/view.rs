#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;

use arangors::view::{ArangoSearchViewLink, ArangoSearchViewPropertiesOptions, ViewOptions};
use arangors::Connection;
use std::collections::HashMap;

const URL: &str = "http://localhost:8529";

#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
#[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
async fn main() -> Result<(), Error> {
    let collection_name = "test_collection".to_string();

    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let database = conn.db("test_db").await?;

    let mut links: HashMap<String, ArangoSearchViewLink> = HashMap::new();

    links.insert(
        collection_name.clone(),
        ArangoSearchViewLink::builder()
            .include_all_fields(true)
            .build(),
    );

    let view = database
        .create_view(
            ViewOptions::builder()
                .name(format!("{}_view", collection_name))
                .properties(
                    ArangoSearchViewPropertiesOptions::builder()
                        .links(links)
                        .build(),
                )
                .build(),
        )
        .await?;

    println!("{:?}", view);

    database
        .drop_view(&format!("{}_view", collection_name))
        .await?;

    Ok(())
}
