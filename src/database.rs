//! struct and enum pertain to arangoDB database
//!
//! AQL query are all executed in database level, so Database offers AQL query.
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use log::trace;
use maybe_async::maybe_async;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::value::Value;
use url::Url;

use crate::{
    aql::{AqlQuery, Cursor},
    client::ClientExt,
    collection::{
        options::{CreateOptions, CreateParameters},
        response::{Info, Properties},
        Collection, CollectionType,
    },
    connection::Version,
    index::{DeleteIndexResponse, Index, IndexCollection},
    response::{deserialize_response, ArangoResult},
    ClientError,
};

#[derive(Debug, Clone)]
pub struct Database<C: ClientExt> {
    name: String,
    base_url: Url,
    session: Arc<C>,
}

impl<'a, C: ClientExt> Database<C> {
    pub(crate) fn new<T: Into<String>>(name: T, arango_url: &Url, session: Arc<C>) -> Database<C> {
        let name = name.into();
        let path = format!("/_db/{}/", name.as_str());
        let url = arango_url.join(path.as_str()).unwrap();
        Database {
            name,
            session,
            base_url: url,
        }
    }

    /// Retrieve all collections of this database.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn accessible_collections(&self) -> Result<Vec<Info>, ClientError> {
        // an invalid arango_url should never running through initialization
        // so we assume arango_url is a valid url
        // When we pass an invalid path, it should panic to eliminate the bug
        // in development.
        let url = self.base_url.join("_api/collection").unwrap();
        trace!(
            "Retrieving collections from {:?}: {}",
            self.name,
            url.as_str()
        );
        let resp = self.session.get(url, "").await?;
        let result: ArangoResult<Vec<Info>> = deserialize_response(resp.body())?;
        trace!("Collections retrieved");
        Ok(result.unwrap())
    }

    pub fn url(&self) -> &Url {
        &self.base_url
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn session(&self) -> Arc<C> {
        Arc::clone(&self.session)
    }

    /// Get collection object with name.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn collection(&self, name: &str) -> Result<Collection<C>, ClientError> {
        let url = self
            .base_url
            .join(&format!("_api/collection/{}", name))
            .unwrap();
        let resp: Info = deserialize_response(self.session.get(url, "").await?.body())?;
        Ok(Collection::from_response(self, &resp))
    }

    /// Create a collection via HTTP request with options.
    ///
    /// Return a collection object if success.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn create_collection_with_options<'f>(
        &self,
        options: CreateOptions<'f>,
        parameters: CreateParameters,
    ) -> Result<Collection<C>, ClientError> {
        let mut url = self.base_url.join("_api/collection").unwrap();
        let query = serde_qs::to_string(&parameters).unwrap();
        url.set_query(Some(query.as_str()));

        let resp = self
            .session
            .post(url, &serde_json::to_string(&options)?)
            .await?;
        let result: Properties = deserialize_response(resp.body())?;
        self.collection(&result.info.name).await
    }

    /// Create a collection via HTTP request.
    ///
    /// Return a collection object if success.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn create_collection(&self, name: &str) -> Result<Collection<C>, ClientError> {
        self.create_collection_with_options(
            CreateOptions::builder().name(name).build(),
            Default::default(),
        )
        .await
    }

    #[maybe_async]
    pub async fn create_edge_collection(&self, name: &str) -> Result<Collection<C>, ClientError> {
        self.create_collection_with_options(
            CreateOptions::builder()
                .name(name)
                .collection_type(CollectionType::Edge)
                .build(),
            Default::default(),
        )
        .await
    }

    /// Drops a collection
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn drop_collection(&self, name: &str) -> Result<String, ClientError> {
        let url_path = format!("_api/collection/{}", name);
        let url = self.base_url.join(&url_path).unwrap();

        #[derive(Debug, Deserialize)]
        struct DropCollectionResponse {
            id: String,
        }

        let resp: DropCollectionResponse =
            deserialize_response(self.session.delete(url, "").await?.body())?;
        Ok(resp.id)
    }

    /// Get the version remote arango database server
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn arango_version(&self) -> Result<Version, ClientError> {
        let url = self.base_url.join("_api/version").unwrap();
        let resp = self.session.get(url, "").await?;
        let version: Version = serde_json::from_str(resp.body())?;
        Ok(version)
    }

    /// Get information of current database.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn info(&self) -> Result<DatabaseDetails, ClientError> {
        let url = self.base_url.join("_api/database/current").unwrap();
        let resp = self.session.get(url, "").await?;
        let res: ArangoResult<DatabaseDetails> = deserialize_response(resp.body())?;
        Ok(res.unwrap())
    }

    /// Execute aql query, return a cursor if succeed. The major advantage of
    /// batch query is that cursors contain more information and stats
    /// about the AQL query, and users can fetch results in batch to save memory
    /// resources on clients.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn aql_query_batch<R>(&self, aql: AqlQuery<'_>) -> Result<Cursor<R>, ClientError>
    where
        R: DeserializeOwned,
    {
        let url = self.base_url.join("_api/cursor").unwrap();
        let resp = self
            .session
            .post(url, &serde_json::to_string(&aql)?)
            .await?;
        deserialize_response(resp.body())
    }

    /// Get next batch given the cursor id.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn aql_next_batch<R>(&self, cursor_id: &str) -> Result<Cursor<R>, ClientError>
    where
        R: DeserializeOwned,
    {
        let url = self
            .base_url
            .join(&format!("_api/cursor/{}", cursor_id))
            .unwrap();
        let resp = self.session.put(url, "").await?;
        deserialize_response(resp.body())
    }

    #[maybe_async]
    async fn aql_fetch_all<R>(&self, response: Cursor<R>) -> Result<Vec<R>, ClientError>
    where
        R: DeserializeOwned,
    {
        let mut response_cursor = response;
        let mut results: Vec<R> = Vec::new();
        loop {
            results.extend(response_cursor.result.into_iter());
            if response_cursor.more {
                let id = response_cursor.id.unwrap().clone();
                response_cursor = self.aql_next_batch(id.as_str()).await?;
            } else {
                break;
            }
        }
        Ok(results)
    }

    /// Execute AQL query fetch all results.
    ///
    /// DO NOT do this when the count of results is too large that network or
    /// memory resources cannot afford.
    ///
    /// DO NOT set a small batch size, otherwise clients will have to make many
    /// HTTP requests.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn aql_query<R>(&self, aql: AqlQuery<'_>) -> Result<Vec<R>, ClientError>
    where
        R: DeserializeOwned,
    {
        let response = self.aql_query_batch(aql).await?;
        if response.more {
            self.aql_fetch_all(response).await
        } else {
            Ok(response.result)
        }
    }

    /// Similar to `aql_query`, except that this method only accept a string of
    /// AQL query.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn aql_str<R>(&self, query: &str) -> Result<Vec<R>, ClientError>
    where
        R: DeserializeOwned,
    {
        let aql = AqlQuery::builder().query(query).build();
        self.aql_query(aql).await
    }

    /// Similar to `aql_query`, except that this method only accept a string of
    /// AQL query, with additional bind vars.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn aql_bind_vars<R>(
        &self,
        query: &str,
        bind_vars: HashMap<&str, Value>,
    ) -> Result<Vec<R>, ClientError>
    where
        R: DeserializeOwned,
    {
        let aql = AqlQuery::builder()
            .query(query)
            .bind_vars(bind_vars)
            .build();
        self.aql_query(aql).await
    }

    /// Create a new index on a collection.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn create_index(
        &self,
        collection: &str,
        index: &Index,
    ) -> Result<Index, ClientError> {
        let mut url = self.base_url.join("_api/index").unwrap();
        url.set_query(Some(&format!("collection={}", collection)));

        let resp = self
            .session
            .post(url, &serde_json::to_string(&index)?)
            .await?;

        let result: Index = deserialize_response::<Index>(resp.body())?;

        Ok(result)
    }

    /// Retrieve an index by id
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn index(&self, id: &str) -> Result<Index, ClientError> {
        let url = self.base_url.join(&format!("_api/index/{}", id)).unwrap();

        let resp = self.session.get(url, "").await?;

        let result: Index = deserialize_response::<Index>(resp.body())?;

        Ok(result)
    }

    /// Retrieve a list of indexes for a collection.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn indexes(&self, collection: &str) -> Result<IndexCollection, ClientError> {
        let mut url = self.base_url.join("_api/index").unwrap();
        url.set_query(Some(&format!("collection={}", collection)));

        let resp = self.session.get(url, "").await?;

        let result: IndexCollection = deserialize_response::<IndexCollection>(resp.body())?;

        Ok(result)
    }

    /// Delete an index by id.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn delete_index(&self, id: &str) -> Result<DeleteIndexResponse, ClientError> {
        let url = self.base_url.join(&format!("_api/index/{}", id)).unwrap();
        let resp = self.session.delete(url, "").await?;

        let result: DeleteIndexResponse = deserialize_response::<DeleteIndexResponse>(resp.body())?;

        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseDetails {
    pub name: String,
    pub id: String,
    pub path: String,
    pub is_system: bool,
}
