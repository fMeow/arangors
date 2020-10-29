#![allow(unused_imports)]
#![allow(unused_parens)]
use log::trace;
use pretty_assertions::assert_eq;

use crate::common::{collection, connection};
use arangors::{
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    transaction::Status as TransactionStatus,
    transaction::{TransactionCollections, TransactionSettings},
    ClientError, Connection, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_start_transaction() {
    test_setup();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();

    let tx_result = database
        .begin_transaction(
            TransactionSettings::builder()
                .lock_timeout(60)
                .collections(
                    TransactionCollections::builder()
                        .write(vec!["test_collection".to_owned()])
                        .build(),
                )
                .build(),
        )
        .await;
    if tx_result.is_err() {
        log::error!("Error: {:?}", tx_result.as_ref().err());
    }
    assert_eq!(tx_result.is_err(), false);

    let tx = tx_result.unwrap();
    let status = tx.abort().await.unwrap();
    trace!("{:?}", status);
    assert_eq!(status, TransactionStatus::Aborted);
}
