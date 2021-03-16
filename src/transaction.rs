use maybe_async::maybe_async;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;
use url::Url;

use crate::{
    aql::Cursor,
    client::ClientExt,
    collection::response::Info,
    response::{deserialize_response, ArangoResult},
    AqlQuery, ClientError, Collection,
};

pub const TRANSACTION_HEADER: &str = "x-arango-trx-id";

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(doc)]
pub struct TransactionCollections {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    read: Option<Vec<String>>,

    write: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct TransactionSettings {
    collections: TransactionCollections,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_sync: Option<bool>,

    #[builder(default = true)]
    allow_implicit: bool,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    lock_timeout: Option<usize>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_transaction_size: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Running,
    Committed,
    Aborted,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArangoTransaction {
    pub id: String,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionState {
    pub id: String,
    pub state: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionList {
    pub transactions: Vec<TransactionState>,
}

/// Represents a [`Transaction`] in ArangoDB.
/// allow you to perform a multi-document transaction with individual begin and commit / abort commands.
/// This is similar to the way traditional RDBMS do it with BEGIN, COMMIT and ROLLBACK operations.
/// # Example
/// ```
/// # use arangors::Connection;
/// # use arangors::Document;
/// # use arangors::transaction::{TransactionCollections, TransactionSettings};
/// # use serde_json::{json, Value};
///
/// # #[cfg_attr(any(feature="reqwest_async"), maybe_async::maybe_async, tokio::main)]
/// # #[cfg_attr(any(feature="surf_async"), maybe_async::maybe_async, async_std::main)]
/// # #[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
/// # async fn main() -> Result<(),anyhow::Error>{
/// # let conn = Connection::establish_jwt("http://localhost:8529", "username", "password")
/// #     .await
/// #     .unwrap();
/// let database = conn.db("test_db").await.unwrap();
///
/// let tx = database.begin_transaction(
///  TransactionSettings::builder()
///      .lock_timeout(60000)
///      .wait_for_sync(true)
///      .collections(
///          TransactionCollections::builder()
///              .write(vec!["test_collection".to_owned()])
///              .build(),
///      )
///     .build(),
///  ).await.unwrap();
///
/// let test_doc: Document<Value> = Document::new(json!({
///   "user_name":"test21",
///  "user_name":"test21_pwd",
/// }));
///
/// let collection = tx.collection("test_collection").await.unwrap();
/// let document = collection
///   .create_document(test_doc, Default::default())
///   .await?;
/// let header = document.header().unwrap();
/// let _key = &header._key;
///
/// tx.abort().await.unwrap();
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Transaction<C: ClientExt> {
    id: String,
    status: Status,
    session: Arc<C>,
    base_url: Url,
}

impl<C> Transaction<C>
where
    C: ClientExt,
{
    pub(crate) fn new(tx: ArangoTransaction, session: Arc<C>, base_url: Url) -> Self {
        Transaction {
            id: tx.id,
            status: tx.status,
            session,
            base_url,
        }
    }

    /// Returns the current transaction status (running, aborted or comitted)
    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Returns the transaction id
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn url(&self) -> &Url {
        &self.base_url
    }

    /// The transaction session, contains the streaming transaction header value
    pub fn session(&self) -> Arc<C> {
        Arc::clone(&self.session)
    }

    /// Tries to commit the transaction, consuming the current object.
    ///
    /// On success all submitted operations will be written in the database and can no longer be aborted.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn commit_transaction(self) -> Result<Status, ClientError> {
        let url = self
            .base_url
            .join(&format!("_api/transaction/{}", self.id))
            .unwrap();

        let resp = self.session.put(url, "").await?;

        let result: ArangoResult<ArangoTransaction> = deserialize_response(resp.body())?;

        Ok(result.unwrap().status)
    }

    /// Tries to commit the transaction.
    ///
    /// On success all submitted operations will be written in the database and can no longer be aborted.
    /// A transaction can be committed multiple times.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn commit(&self) -> Result<Status, ClientError> {
        let url = self
            .base_url
            .join(&format!("_api/transaction/{}", self.id))
            .unwrap();

        let resp = self.session.put(url, "").await?;

        let result: ArangoResult<ArangoTransaction> = deserialize_response(resp.body())?;

        Ok(result.unwrap().status)
    }

    /// Tries to abort the transaction.
    ///
    /// On success all submitted operations will be cancelled and can no longer be committed.
    /// A ransaction can be aborted multiple times without error.
    ///
    /// # Warning
    ///
    /// If the transaction is aborted, then it means deletion on the server side. The current object
    /// can no longer be used for operations or commit.
    ///
    /// # Note
    /// this function would make a request to arango server.
    #[maybe_async]
    pub async fn abort(&self) -> Result<Status, ClientError> {
        let url = self
            .base_url
            .join(&format!("_api/transaction/{}", self.id))
            .unwrap();

        let resp = self.session.delete(url, "").await?;

        let result: ArangoResult<ArangoTransaction> = deserialize_response(resp.body())?;

        Ok(result.unwrap().status)
    }

    /// Get collection object with name.
    ///
    /// The returned collection object will share its session with the transaction, meaning all
    /// operations using the colleciton will be transactional and require a transaction commit to be writen
    /// in ArangoDB.
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
        Ok(Collection::from_transaction_response(self, &resp))
    }

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
            if response_cursor.more {
                let id = response_cursor.id.unwrap().clone();
                results.extend(response_cursor.result.into_iter());
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
}
