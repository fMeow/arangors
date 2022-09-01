#![allow(unused_imports)]
#![allow(unused_parens)]
use crate::common::{collection, connection};

use log::{info, trace};
use maybe_async::maybe_async;
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use uclient::ClientExt;

use arangors::analyzer::{
    AnalyzerCase, AnalyzerFeature, AnalyzerInfo, NgramAnalyzerProperties, NgramStreamType,
    NormAnalyzerProperties, GeoJsonAnalyzerProperties, GeoJsonType,
};
use arangors::{
    collection::{
        options::{ChecksumOptions, PropertiesOptions},
        response::Status,
        CollectionType,
    },
    view::View,
    ClientError, Connection, Database, Document,
};
use common::{get_arangodb_host, get_normal_password, get_normal_user, test_setup};

pub mod common;

#[maybe_async]
async fn create_norm_analyzer<C: ClientExt>(
    database: &Database<C>,
    analyzer_name: String,
) -> Result<AnalyzerInfo, ClientError> {
    let info = AnalyzerInfo::Norm {
        name: analyzer_name,
        features: Some(vec![AnalyzerFeature::Frequency, AnalyzerFeature::Norm]),
        properties: Some(
            NormAnalyzerProperties::builder()
                .locale("en.utf-8".to_string())
                .case(AnalyzerCase::Lower)
                .build(),
        ),
    };

    database.create_analyzer(info).await
}

#[maybe_async]
async fn create_ngram_analyzer<C: ClientExt>(
    database: &Database<C>,
    analyzer_name: String,
) -> Result<AnalyzerInfo, ClientError> {
    let info = AnalyzerInfo::Ngram {
        name: analyzer_name,
        features: Some(vec![AnalyzerFeature::Frequency, AnalyzerFeature::Norm]),
        properties: Some(
            NgramAnalyzerProperties::builder()
                .min(2)
                .max(2)
                .preserve_original(false)
                .stream_type(NgramStreamType::Utf8)
                .build(),
        ),
    };

    database.create_analyzer(info).await
}

#[maybe_async]
async fn create_geo_analyzer<C: ClientExt>(
    database: &Database<C>,
    analyzer_name: String,
) -> Result<AnalyzerInfo, ClientError> {
    let info = AnalyzerInfo::Geojson {
        name: analyzer_name,
        features: Some(vec![AnalyzerFeature::Frequency, AnalyzerFeature::Norm]),
        properties: Some(
            GeoJsonAnalyzerProperties::builder()
                .r#type(GeoJsonType::Centroid)
                .build(),
        ),
    };

    database.create_analyzer(info).await
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_create_and_drop_norm_analyzer() {
    test_setup();
    let analyzer_name = "test_analyzer_norm_create".to_string();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();

    let analyzer = create_norm_analyzer(&database, analyzer_name.clone()).await;

    trace!("{:?}", analyzer);

    assert_eq!(analyzer.is_err(), false);

    let result = database.drop_analyzer(&analyzer_name).await;

    assert_eq!(result.is_err(), false);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_create_and_drop_ngram_analyzer() {
    test_setup();
    let analyzer_name = "test_analyzer_ngram_create".to_string();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();

    let analyzer = create_ngram_analyzer(&database, analyzer_name.clone()).await;

    trace!("{:?}", analyzer);

    assert_eq!(analyzer.is_err(), false);

    let result = database.drop_analyzer(&analyzer_name).await;

    assert_eq!(result.is_err(), false);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_create_and_drop_geo_analyzer() {
    test_setup();
    let analyzer_name = "test_analyzer_geo_create".to_string();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();

    let analyzer = create_geo_analyzer(&database, analyzer_name.clone()).await;

    trace!("{:?}", analyzer);

    assert_eq!(analyzer.is_err(), false);

    let result = database.drop_analyzer(&analyzer_name).await;

    assert_eq!(result.is_err(), false);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_list_analyzer() {
    test_setup();
    let analyzer_name = "test_analyzer_list".to_string();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();

    let analyzer = create_norm_analyzer(&database, analyzer_name.clone()).await;

    trace!("{:?}", analyzer);

    assert_eq!(analyzer.is_err(), false);
    let analyzer = analyzer.unwrap();

    let analyzers = database.list_analyzers().await;

    let views_analyzers = analyzers.unwrap();

    let analyzer_found = views_analyzers.iter().find(|a| **a == analyzer);

    assert_eq!(analyzer_found.is_some(), true);

    let result = database.drop_analyzer(&analyzer_name).await;

    assert_eq!(result.is_err(), false);
}

#[maybe_async::test(
    any(feature = "reqwest_blocking"),
    async(any(feature = "reqwest_async"), tokio::test),
    async(any(feature = "surf_async"), async_std::test)
)]
async fn test_create_and_exists() {
    test_setup();
    let analyzer_name = "test_analyzer_exists".to_string();
    let conn = connection().await;
    let database = conn.db("test_db").await.unwrap();

    let analyzer = create_norm_analyzer(&database, analyzer_name.clone()).await;

    trace!("{:?}", analyzer);

    assert_eq!(analyzer.is_err(), false);

    let queried_analyzer = database.analyzer(&analyzer_name).await;

    assert_eq!(queried_analyzer.is_err(), false);

    assert_eq!(analyzer.unwrap(), queried_analyzer.unwrap());

    let result = database.drop_analyzer(&analyzer_name).await;

    assert_eq!(result.is_err(), false);
}
