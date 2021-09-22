#![allow(unused_imports)]
#![allow(unused_parens)]
use log::trace;
use maybe_async::maybe_async;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

use crate::common::{collection, connection};
use arangors::{
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    document::options::RemoveOptions,
    transaction::Status as TransactionStatus,
    transaction::{Transaction, TransactionCollections, TransactionSettings},
    ClientError, Connection, Database, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;

#[maybe_async]
async fn create_transaction(
    database: &Database,
    collection_name: String,
) -> Result<Transaction, ClientError> {
    database
        .begin_transaction(
            TransactionSettings::builder()
                .lock_timeout(60)
                .collections(
                    TransactionCollections::builder()
                        .write(vec![collection_name])
                        .build(),
                )
                .build(),
        )
        .await
}

#[maybe_async]
async fn create_document(tx: &Transaction) -> Result<String, ClientError> {
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
    Ok(_key.clone())
}

#[maybe_async::test(
    feature = "blocking",
    async(not(feature = "blocking"), tokio::test),
)]
async fn test_start_transaction() {
    test_setup();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();
    let tx_result = create_transaction(&database, "test_collection".to_string()).await;

    if tx_result.is_err() {
        log::error!("Error: {:?}", tx_result.as_ref().err());
    }
    assert_eq!(tx_result.is_err(), false);

    let tx = tx_result.unwrap();
    let status = tx.abort().await.unwrap();
    trace!("{:?}", status);
    assert_eq!(status, TransactionStatus::Aborted);
}

#[maybe_async::test(
    feature = "blocking",
    async(not(feature = "blocking"), tokio::test),
)]
async fn test_abort_transaction() {
    test_setup();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();
    let tx_result = create_transaction(&database, "test_collection".to_string()).await;

    if tx_result.is_err() {
        log::error!("Error: {:?}", tx_result.as_ref().err());
    }
    assert_eq!(tx_result.is_err(), false);

    let tx = tx_result.unwrap();

    let key_result = create_document(&tx).await;

    assert_eq!(key_result.is_err(), false);

    let status = tx.abort().await.unwrap();
    trace!("{:?}", status);
    assert_eq!(status, TransactionStatus::Aborted);

    let key = key_result.unwrap();
    let collection = database.collection("test_collection").await.unwrap();
    let doc = collection.document::<Value>(&key).await;

    assert_eq!(doc.is_err(), true);
}

#[maybe_async::test(
    feature = "blocking",
    async(not(feature = "blocking"), tokio::test),
)]
async fn test_commit_transaction() {
    test_setup();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();
    let tx_result = create_transaction(&database, "test_collection".to_string()).await;

    if tx_result.is_err() {
        log::error!("Error: {:?}", tx_result.as_ref().err());
    }
    assert_eq!(tx_result.is_err(), false);

    let tx = tx_result.unwrap();

    let key_result = create_document(&tx).await;

    assert_eq!(key_result.is_err(), false);

    let status = tx.commit().await.unwrap();
    trace!("{:?}", status);
    assert_eq!(status, TransactionStatus::Committed);

    let key = key_result.unwrap();
    let collection = database.collection("test_collection").await.unwrap();
    let doc = collection.document::<Value>(&key).await;

    assert_eq!(doc.is_ok(), true);

    let old_doc = collection
        .remove_document::<Value>(
            &key,
            RemoveOptions::builder().return_old(true).build(),
            None,
        )
        .await;

    assert_eq!(old_doc.is_ok(), true);
}
