#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;

use arangors::analyzer::{AnalyzerCase, AnalyzerFeature, AnalyzerInfo, NormAnalyzerProperties};
use arangors::Connection;
use std::collections::HashMap;

const URL: &str = "http://localhost:8529";

#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
#[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
async fn main() -> Result<(), Error> {
    let analyzer_name = "test_analyzer".to_string();

    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let database = conn.db("test_db").await?;

    let info = AnalyzerInfo::Norm {
        name: analyzer_name.clone(),
        features: Some(vec![AnalyzerFeature::Frequency, AnalyzerFeature::Norm]),
        properties: Some(
            NormAnalyzerProperties::builder()
                .locale("en.utf-8".to_string())
                .case(AnalyzerCase::Lower)
                .build(),
        ),
    };

    database.create_analyzer(info).await?;

    database.drop_analyzer(&analyzer_name).await?;

    Ok(())
}
