#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;
use arangors::Document;
use arangors::{
    transaction::{TransactionCollections, TransactionSettings},
    Connection,
};
use log::info;
use serde_json::{json, Value};

const URL: &str = "http://localhost:8529";

#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
#[cfg_attr(feature = "reqwest_blocking", maybe_async::must_be_sync)]
async fn main() -> Result<(), Error> {
    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let database = conn.db("test_db").await?;

    let tx = database
        .begin_transaction(
            TransactionSettings::builder()
                .lock_timeout(60000)
                .wait_for_sync(true)
                .collections(
                    TransactionCollections::builder()
                        .write(vec!["test_collection".to_owned()])
                        .build(),
                )
                .build(),
        )
        .await?;

    let transactions = database.list_transactions().await?;
    info!("Transactions: {:?}", transactions);

    let test_doc: Document<Value> = Document::new(json!({
        "user_name":"test21",
        "user_name":"test21_pwd",
    }));

    let collection = tx.collection("test_collection").await?;
    let document = collection
        .create_document(test_doc, Default::default())
        .await?;
    let header = document.header().unwrap();
    let _key = &header._key;

    info!("Key: {}", _key);
    tx.abort().await?;

    Ok(())
}
