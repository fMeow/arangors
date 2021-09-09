/// Types related to AQL query in arangoDB.
///
/// While aql queries are performed on database, it would be ponderous to
/// place all aql query related methods and types in `arangors::database`.
///
/// Steps to perform a AQL query:
/// 1. (optional) construct a AqlQuery object.
///     - (optional) construct AqlOption.
/// 1. perform AQL query via `database.aql_query`.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, TypedBuilder)]
#[builder(
    doc,
    builder_method_doc = r#"Create a builder for building `AqlQuery`.

On the builder, call `.query(...)`, `.bind_vars(...)(optional)`, `.bind_var(...)(optional)`,
`.try_bind(...)(optional)`, `.count(...)(optional)`, `.batch_size(...)(optional)`,
`.cache(...)(optional)`, `.memory_limit(...)(optional)`, `.ttl(...)(optional)`,
`.options(...)(optional)` to set the values of the fields (they accept Into values).

Use `.try_bind(...)` to accept any serializable struct
while `.bind_value(...)` accepts an `Into<serde_json::Value>`.

Finally, call .build() to create the instance of AqlQuery."#
)]
#[serde(rename_all = "camelCase")]
pub struct AqlQuery<'a> {
    /// query string to be executed
    query: &'a str,

    /// bind parameters to substitute in query string
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[builder(default)]
    bind_vars: HashMap<&'a str, Value>,

    /// Indicates whether the number of documents in the result set should be
    /// returned in the "count" attribute of the result.
    ///
    /// Calculating the 'count' attribute might have a performance impact
    /// for some queries in the future so this option is turned off by default,
    /// and 'count' is only returned when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    count: Option<bool>,

    /// Maximum number of result documents to be transferred from the server to
    /// the client in one round-trip.
    ///
    /// If this attribute is not set, a server-controlled default value will
    /// be used.
    ///
    /// A batchSize value of 0 is disallowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    batch_size: Option<u32>,

    /// A flag to determine whether the AQL query cache shall be used.
    ///
    /// If set to false, then any query cache lookup will be skipped for the
    /// query. If set to true, it will lead to the query cache being
    /// checked for the query if the query cache mode is either on or
    /// demand.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    cache: Option<bool>,

    /// The maximum number of memory (measured in bytes) that the query is
    /// allowed to use.
    ///
    /// If set, then the query will fail with error 'resource
    /// limit exceeded' in case it allocates too much memory.
    ///
    /// A value of 0 indicates that there is no memory limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    memory_limit: Option<u64>,

    /// The time-to-live for the cursor (in seconds).
    ///
    /// The cursor will be removed on the server automatically after
    /// the specified amount of time. This is useful to ensure garbage
    /// collection of cursors that are not fully fetched by clients.
    ///
    /// If not set, a server-defined value will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    ttl: Option<u32>,

    /// Options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    options: Option<AqlOptions>,
}

// when binding the first query variable
#[allow(non_camel_case_types, missing_docs)]
impl<'a, __query, __count, __batch_size, __cache, __memory_limit, __ttl, __options>
    AqlQueryBuilder<
        'a,
        (
            __query,
            (),
            __count,
            __batch_size,
            __cache,
            __memory_limit,
            __ttl,
            __options,
        ),
    >
{
    #[allow(clippy::type_complexity)]
    pub fn bind_var<K, V>(
        self,
        key: K,
        value: V,
    ) -> AqlQueryBuilder<
        'a,
        (
            __query,
            (HashMap<&'a str, Value>,),
            __count,
            __batch_size,
            __cache,
            __memory_limit,
            __ttl,
            __options,
        ),
    >
    where
        K: Into<&'a str>,
        V: Into<Value>,
    {
        let mut bind_vars = HashMap::new();
        bind_vars.insert(key.into(), value.into());
        let (query, _, count, batch_size, cache, memory_limit, ttl, options) = self.fields;
        AqlQueryBuilder {
            fields: (
                query,
                (bind_vars,),
                count,
                batch_size,
                cache,
                memory_limit,
                ttl,
                options,
            ),
            phantom: self.phantom,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn try_bind<K, V>(
        self,
        key: K,
        value: V,
    ) -> Result<
        AqlQueryBuilder<
            'a,
            (
                __query,
                (HashMap<&'a str, Value>,),
                __count,
                __batch_size,
                __cache,
                __memory_limit,
                __ttl,
                __options,
            ),
        >,
        serde_json::Error,
    >
    where
        K: Into<&'a str>,
        V: serde::Serialize,
    {
        Ok(self.bind_var(key, serde_json::to_value(value)?))
    }
}

// when bind_var(s) are not empty
#[allow(non_camel_case_types, missing_docs)]
impl<'a, __query, __count, __batch_size, __cache, __memory_limit, __ttl, __options>
    AqlQueryBuilder<
        'a,
        (
            __query,
            (HashMap<&'a str, Value>,),
            __count,
            __batch_size,
            __cache,
            __memory_limit,
            __ttl,
            __options,
        ),
    >
{
    #[allow(clippy::type_complexity)]
    pub fn bind_var<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> AqlQueryBuilder<
        'a,
        (
            __query,
            (HashMap<&'a str, Value>,),
            __count,
            __batch_size,
            __cache,
            __memory_limit,
            __ttl,
            __options,
        ),
    >
    where
        K: Into<&'a str>,
        V: Into<Value>,
    {
        (self.fields.1).0.insert(key.into(), value.into());
        self
    }

    #[allow(clippy::type_complexity)]
    pub fn try_bind<K, V>(
        self,
        key: K,
        value: V,
    ) -> Result<
        AqlQueryBuilder<
            'a,
            (
                __query,
                (HashMap<&'a str, Value>,),
                __count,
                __batch_size,
                __cache,
                __memory_limit,
                __ttl,
                __options,
            ),
        >,
        serde_json::Error,
    >
    where
        K: Into<&'a str>,
        V: serde::Serialize,
    {
        Ok(self.bind_var(key, serde_json::to_value(value)?))
    }
}

#[derive(Debug, Serialize, TypedBuilder, PartialEq)]
#[builder(doc)]
#[serde(rename_all = "camelCase")]
pub struct AqlOptions {
    /// When set to true, the query will throw an exception and abort instead of
    /// producing a warning.
    ///
    /// This option should be used during development to catch potential issues
    /// early.
    ///
    /// When the attribute is set to false, warnings will not be propagated to
    /// exceptions and will be returned with the query result.
    /// There is also a server configuration option `--query.fail-on-warning`
    ///  for setting the default value for `fail_on_warning` so it does not
    /// need to be set on a per-query level.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    fail_on_warning: Option<bool>,

    /// If set to true, then the additional query profiling information will
    /// be returned in the sub-attribute profile of the extra return attribute
    /// if the query result is not served from the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    profile: Option<bool>,

    /// Limits the maximum number of warnings a query will return.
    ///
    /// The number of warnings a query will return is limited to 10 by default,
    /// but that number can be increased or decreased by setting this attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    max_warning_count: Option<u32>,

    /// If set to true and the query contains a LIMIT clause, then the result
    /// will have an extra attribute with the sub-attributes stats and
    /// fullCount, `{ ... , "extra": { "stats": { "fullCount": 123 } } }`.
    ///
    /// The fullCount attribute will contain the number of documents in the
    /// result before the last LIMIT in the query was applied. It can be
    /// used to count the number of documents that match certain filter
    /// criteria, but only return a subset of them, in one go. It is thus
    /// similar to MySQL's `SQL_CALC_FOUND_ROWS` hint. Note that setting
    /// the option will disable a few LIMIT optimizations and may lead to
    /// more documents being processed, and thus make queries run longer.
    /// Note that the fullCount attribute
    /// will only be present in the result if the query has a LIMIT clause
    /// and the LIMIT clause is actually used in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    full_count: Option<bool>,

    /// Limits the maximum number of plans that are created by the AQL query
    /// optimizer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    max_plans: Option<u32>,

    /// A list string indicating to-be-included or to-be-excluded optimizer
    /// rules can be put into this attribute, telling the optimizer to
    /// include or exclude specific rules.
    ///
    /// To disable a rule, prefix its name with a `-`.
    ///
    /// To enable a rule, prefix it with a `+`.
    ///
    /// There is also a pseudo-rule `"all"`, which will match all optimizer
    /// rules.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    optimizer: Vec<String>,

    /// Maximum number of operations after which an intermediate commit is
    /// performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    intermediate_commit_count: Option<u32>,

    /// Maximum total size of operations after which an intermediate commit is
    /// performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    intermediate_commit_size: Option<u32>,

    /// Transaction size limit in bytes.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    max_transaction_size: Option<u32>,

    /// This enterprise parameter allows to configure how long a DBServer will
    /// have time to bring the satellite collections involved in the query into
    /// sync.
    ///
    /// The default value is 60.0 (seconds). When the max time has been
    /// reached the query will be stopped.
    #[cfg(feature = "enterprise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    satellite_sync_wait: Option<bool>,
}

impl Default for AqlOptions {
    fn default() -> AqlOptions {
        Self::builder().build()
    }
}

impl AqlOptions {
    pub fn set_optimizer(&mut self, optimizer: String) {
        self.optimizer.push(optimizer)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryStats {
    /// The total number of data-modification operations successfully executed.
    ///
    /// This is equivalent to the number of documents created, updated or
    /// removed by `INSERT`, `UPDATE`, `REPLACE` or `REMOVE` operations.
    pub writes_executed: usize,

    /// Total number of data-modification operations that were unsuccessful,
    /// but have been ignored because of query option ignoreErrors.
    pub writes_ignored: usize,

    /// Total number of documents iterated over when scanning a collection
    /// without an index.
    ///
    /// Documents scanned by subqueries will be included in the result, but not
    /// no operations triggered by built-in or user-defined AQL functions.
    pub scanned_full: usize,
    /// Total number of documents iterated over when scanning a collection
    /// using an index.
    ///
    /// Documents scanned by subqueries will be included in the result, but not
    /// no operations triggered by built-in or user-defined AQL functions.
    pub scanned_index: usize,
    /// Total number of documents that were removed after executing a filter
    /// condition in a FilterNode.
    ///
    /// Note that IndexRangeNodes can also filter documents by selecting only
    /// the required index range from a collection, and the filtered value
    /// only indicates how much filtering was done by FilterNodes.
    pub filtered: usize,

    /// Total number of documents that matched the search condition if the
    /// query's final LIMIT statement were not present.
    ///
    /// This attribute will only be returned if the fullCount option was set
    /// when starting the query and will only contain a sensible value if the
    /// query contained a LIMIT operation on the top level.
    pub full_count: Option<usize>,
    pub http_requests: usize,
    pub execution_time: f64,
}

#[derive(Deserialize, Debug)]
pub struct Cursor<T> {
    /// the total number of result documents available
    ///
    /// only available if the query was executed with the count attribute
    /// set
    pub count: Option<usize>,
    /// a boolean flag indicating whether the query result was served from
    /// the query cache or not.
    ///
    /// If the query result is served from the query cache, the extra
    /// return attribute will not contain any stats sub-attribute
    /// and no profile sub-attribute.,
    pub cached: bool,
    /// A boolean indicator whether there are more results available for
    /// the cursor on the server
    #[serde(rename = "hasMore")]
    pub more: bool,

    /// (anonymous json object): an array of result documents (might be
    /// empty if query has no results)
    pub result: Vec<T>,
    ///  id of temporary cursor created on the server
    pub id: Option<String>,

    /// an optional JSON object with extra information about the query
    /// result contained in its stats sub-attribute. For
    /// data-modification queries, the extra.stats sub-attribute
    /// will contain the number of
    /// modified documents and the number of documents that could
    /// not be modified due to an error if ignoreErrors query
    /// option is specified.
    pub extra: Option<QueryExtra>,
}

#[derive(Deserialize, Debug)]
pub struct QueryExtra {
    // TODO
    pub stats: Option<QueryStats>,
    // TODO
    pub warnings: Option<Vec<Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aql_query_builder_bind_var() {
        let q = r#"FOR i in test_collection FILTER i.username==@username AND i.password==@password return i"#;
        let aql = AqlQuery::builder()
            .query(q)
            // test the first bind
            .bind_var("username", "test2")
            // test the second bind
            .bind_var("password", "test2_pwd")
            .count(true)
            .batch_size(256)
            .cache(false)
            .memory_limit(100)
            .ttl(10)
            .build();
        assert_eq!(aql.query, q);
        assert_eq!(aql.count, Some(true));
        assert_eq!(aql.batch_size, Some(256u32));
        assert_eq!(aql.cache, Some(false));
        assert_eq!(aql.memory_limit, Some(100));
        assert_eq!(aql.ttl, Some(10));
        assert_eq!(aql.options, None);

        assert_eq!(
            aql.bind_vars.get("username"),
            Some(&Value::String("test2".to_owned()))
        );
        assert_eq!(
            aql.bind_vars.get("password"),
            Some(&Value::String("test2_pwd".to_owned()))
        );
    }

    #[test]
    fn aql_query_builder_try_bind() {
        #[derive(Serialize, Deserialize, Debug)]
        struct User {
            pub username: String,
            pub password: String,
        }
        let user = User {
            username: "test2".to_owned(),
            password: "test2_pwd".to_owned(),
        };
        let q = r#"FOR i in test_collection FILTER i==@user return i"#;
        let aql = AqlQuery::builder()
            .query(q)
            .try_bind("user", user)
            .unwrap()
            .build();

        assert_eq!(aql.query, q);
        assert_eq!(aql.count, None);
        assert_eq!(aql.batch_size, None);

        let mut map = serde_json::Map::new();
        map.insert("username".into(), "test2".into());
        map.insert("password".into(), "test2_pwd".into());

        assert_eq!(aql.bind_vars.get("user"), Some(&Value::Object(map)));

        let aql = AqlQuery::builder()
            .query(r#"FOR i in test_collection FILTER i.username==@username AND i.password==@password return i"#)
            // test the first bind
            .try_bind("username", "test2")
            .unwrap()
            // test the second bind
            .try_bind("password", "test2_pwd")
            .unwrap()
            .build();

        assert_eq!(
            aql.bind_vars.get("username"),
            Some(&Value::String("test2".to_owned()))
        );
        assert_eq!(
            aql.bind_vars.get("password"),
            Some(&Value::String("test2_pwd".to_owned()))
        );
    }
}
