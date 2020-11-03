#![allow(unused_imports)]
#![allow(unused_parens)]
use crate::common::{collection, connection};

use arangors::view::{ArangoSearchViewLink, ArangoSearchViewPropertiesOptions, ViewOptions};
use arangors::{
    client::ClientExt,
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    view::View,
    ClientError, Connection, Database, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};
use log::{info, trace};
use maybe_async::maybe_async;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

pub mod common;

#[maybe_async]
async fn create_view<C: ClientExt>(
    database: &Database<C>,
    view_name: String,
    collection_name: String,
) -> Result<View, ClientError> {
    let mut links: HashMap<String, ArangoSearchViewLink> = HashMap::new();

    links.insert(
        collection_name,
        ArangoSearchViewLink::builder()
            .include_all_fields(true)
            .build(),
    );

    database
        .create_view(
            ViewOptions::builder()
                .name(view_name)
                .properties(
                    ArangoSearchViewPropertiesOptions::builder()
                        .links(links)
                        .build(),
                )
                .build(),
        )
        .await
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_create_and_drop_view() {
    test_setup();
    let collection_name = "test_collection".to_string();
    let view_name = format!("{}_view_create", collection_name);
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();

    let view = create_view(&database, view_name, collection_name.clone()).await;

    trace!("{:?}", view);

    assert_eq!(view.is_err(), false);

    let result = database
        .drop_view(&format!("{}_view_create", collection_name))
        .await;

    assert_eq!(result.is_err(), false);
}

// #[maybe_async::test(
//     any(feature = "reqwest_blocking"),
//     async(any(feature = "reqwest_async"), tokio::test),
//     async(any(feature = "surf_async"), async_std::test)
// )]
// async fn test_list_view() {
//     test_setup();
//     let collection_name = "test_collection".to_string();
//     let view_name = format!("{}_view_list", collection_name);
//     let conn = connection().await;
//     let database = conn.db("test_db").await.unwrap();

//     let view = create_view(&database, view_name.clone(), collection_name.clone()).await;

//     trace!("{:?}", view);

//     assert_eq!(view.is_err(), false);

//     let views = database.list_views().await;

//     trace!("{:?}", views);
//     assert_eq!(views.is_err(), false);

//     let views_list = views.unwrap();

//     let view_found = views_list.iter().find(|vd| vd.name == view_name.clone());

//     assert_eq!(view_found.is_some(), true);

//     let result = database
//         .drop_view(&format!("{}_view_list", collection_name))
//         .await;

//     assert_eq!(result.is_err(), false);
// }

// #[maybe_async::test(
//     any(feature = "reqwest_blocking"),
//     async(any(feature = "reqwest_async"), tokio::test),
//     async(any(feature = "surf_async"), async_std::test)
// )]
// async fn update_properties() {
//     test_setup();
//     let collection_name = "test_collection".to_string();
//     let view_name = format!("{}_view_update", collection_name);
//     let conn = connection().await;
//     let database = conn.db("test_db").await.unwrap();

//     let view = create_view(&database, view_name.clone(), collection_name.clone()).await;

//     trace!("{:?}", view);

//     assert_eq!(view.is_err(), false);

//     let updated_view = database
//         .update_view_properties(
//             &view_name,
//             ArangoSearchViewPropertiesOptions::builder()
//                 .cleanup_interval_step(3)
//                 .build(),
//         )
//         .await;

//     trace!("{:?}", updated_view);

//     assert_eq!(updated_view.is_err(), false);

//     let result = database
//         .drop_view(&format!("{}_view_update", collection_name))
//         .await;

//     assert_eq!(result.is_err(), false);
// }
