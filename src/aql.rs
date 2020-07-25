/// Types related to AQL query in arangoDB.
///
/// While aql queries are performed on database, it would be ponderous to
/// place all aql query related methods and types in `arangors::database`.
///
/// Steps to perform a AQL query:
/// 1. (optional) construct a AqlQuery object.
///     - (optional) construct AqlOption.
/// 1. (TODO) locally validate aql queries.
/// 1. perform AQL query via `self.session`.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AqlQuery<'a> {
    /// Indicates whether this query is valid.
    ///
    /// Note that the validation is performed locally.
    #[serde(skip_serializing)]
    valid: Option<bool>,

    /// query string to be executed
    query: &'a str,

    /// bind parameters to substitute in query string
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    bind_vars: HashMap<String, Value>,

    /// Indicates whether the number of documents in the result set should be
    /// returned in the "count" attribute of the result.
    ///
    /// Calculating the 'count' attribute might have a performance impact
    /// for some queries in the future so this option is turned off by default,
    /// and 'count' is only returned when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<bool>,

    /// Maximum number of result documents to be transferred from the server to
    /// the client in one round-trip.
    ///
    /// If this attribute is not set, a server-controlled default value will
    /// be used.
    ///
    /// A batchSize value of 0 is disallowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<u32>,

    /// A flag to determine whether the AQL query cache shall be used.
    ///
    /// If set to false, then any query cache lookup will be skipped for the
    /// query. If set to true, it will lead to the query cache being
    /// checked for the query if the query cache mode is either on or
    /// demand.
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<bool>,

    /// The maximum number of memory (measured in bytes) that the query is
    /// allowed to use.
    ///
    /// If set, then the query will fail with error 'resource
    /// limit exceeded' in case it allocates too much memory.
    ///
    /// A value of 0 indicates that there is no memory limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_limit: Option<u64>,

    /// The time-to-live for the cursor (in seconds).
    ///
    /// The cursor will be removed on the server automatically after
    /// the specified amount of time. This is useful to ensure garbage
    /// collection of cursors that are not fully fetched by clients.
    ///
    /// If not set, a server-defined value will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u32>,

    /// Options
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<AqlOption>,
}

impl<'a> Default for AqlQuery<'a> {
    fn default() -> AqlQuery<'a> {
        AqlQuery {
            query: "",
            valid: Some(false),
            bind_vars: HashMap::new(),
            count: None,
            batch_size: None,
            cache: None,
            memory_limit: None,
            ttl: None,
            options: None,
        }
    }
}

impl<'a> AqlQuery<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            ..Default::default()
        }
    }

    pub fn bind_var<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.bind_vars.insert(key.into(), value.into());
        self
    }

    pub fn count(mut self, option: bool) -> Self {
        self.count = Some(option);
        self
    }

    pub fn batch_size(mut self, option: u32) -> Self {
        self.batch_size = Some(option);
        self
    }

    pub fn cache(mut self, option: bool) -> Self {
        self.cache = Some(option);
        self
    }

    pub fn memory_limit(mut self, option: u64) -> Self {
        self.memory_limit = Some(option);
        self
    }

    pub fn ttl(mut self, option: u32) -> Self {
        self.ttl = Some(option);
        self
    }

    pub fn options(mut self, option: AqlOption) -> Self {
        self.options = Some(option);
        self
    }

    // fn is_valid(&self) -> bool {
    //     match self.valid{
    //         Some(valid)=>valid,
    //         None=>{self.check()},
    //     }
    // }
    // fn check(&mut self)->Result<&mut AqlQuery,Error>{
    //     if self.query.len()==0{
    //         Err(format_err!("Query should not be empty"))
    //     }
    //     else{
    //         match self.bind_vars{
    //             Some(vars)=>{
    //                 vars.iter().map(|(key,value)|{
    //                     let re = Regex::new(r"@"+&key.as_str()).unwrap();
    //                     if !re.is_match(){
    //                         // TODO
    //                         Err(format_err!(""))
    //                     }
    //                 });
    //     }

    // }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AqlOption {
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
    pub fail_on_warning: Option<bool>,

    /// If set to true, then the additional query profiling information will
    /// be returned in the sub-attribute profile of the extra return attribute
    /// if the query result is not served from the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<bool>,

    /// Limits the maximum number of warnings a query will return.
    ///
    /// The number of warnings a query will return is limited to 10 by default,
    /// but that number can be increased or decreased by setting this attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_warning_count: Option<u32>,

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
    pub full_count: Option<bool>,

    /// Limits the maximum number of plans that are created by the AQL query
    /// optimizer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_plans: Option<u32>,

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
    pub optimizer: Vec<String>,

    /// Maximum number of operations after which an intermediate commit is
    /// performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_commit_count: Option<u32>,

    /// Maximum total size of operations after which an intermediate commit is
    /// performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_commit_size: Option<u32>,

    /// Transaction size limit in bytes.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_transaction_size: Option<u32>,

    /// This enterprise parameter allows to configure how long a DBServer will
    /// have time to bring the satellite collections involved in the query into
    /// sync.
    ///
    /// The default value is 60.0 (seconds). When the max time has been
    /// reached the query will be stopped.
    #[cfg(feature = "enterprise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellite_sync_wait: Option<bool>,
}

impl Default for AqlOption {
    fn default() -> AqlOption {
        AqlOption {
            fail_on_warning: None,
            profile: None,
            max_warning_count: None,
            full_count: None,
            max_plans: None,
            optimizer: Vec::new(),
            #[cfg(feature = "rocksdb")]
            intermediate_commit_count: None,
            #[cfg(feature = "rocksdb")]
            intermediate_commit_size: None,
            #[cfg(feature = "rocksdb")]
            max_transaction_size: None,
            #[cfg(feature = "enterprise")]
            satellite_sync_wait: None,
        }
    }
}
impl AqlOption {
    pub fn set_optimizer(&mut self, optimizer: String) {
        self.optimizer.push(optimizer)
    }
}

#[derive(Debug, Deserialize)]
struct QueryExtra {
    stats: Option<QueryStats>,
    warnings: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryStats {
    /// The total number of data-modification operations successfully executed.
    ///
    /// This is equivalent to the number of documents created, updated or
    /// removed by `INSERT`, `UPDATE`, `REPLACE` or `REMOVE` operations.
    writes_executed: usize,

    /// Total number of data-modification operations that were unsuccessful,
    /// but have been ignored because of query option ignoreErrors.
    writes_ignored: usize,

    /// Total number of documents iterated over when scanning a collection
    /// without an index.
    ///
    /// Documents scanned by subqueries will be included in the result, but not
    /// no operations triggered by built-in or user-defined AQL functions.
    scanned_full: usize,
    /// Total number of documents iterated over when scanning a collection
    /// using an index.
    ///
    /// Documents scanned by subqueries will be included in the result, but not
    /// no operations triggered by built-in or user-defined AQL functions.
    scanned_index: usize,
    /// Total number of documents that were removed after executing a filter
    /// condition in a FilterNode.
    ///
    /// Note that IndexRangeNodes can also filter documents by selecting only
    /// the required index range from a collection, and the filtered value
    /// only indicates how much filtering was done by FilterNodes.
    filtered: usize,

    /// Total number of documents that matched the search condition if the
    /// query's final LIMIT statement were not present.
    ///
    /// This attribute will only be returned if the fullCount option was set
    /// when starting the query and will only contain a sensible value if the
    /// query contained a LIMIT operation on the top level.
    full_count: Option<usize>,
    http_requests: usize,
    execution_time: f64,
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
    pub extra: Option<Extra>,
}

#[derive(Deserialize, Debug)]
pub struct Extra {
    // TODO
    stats: Option<QueryStats>,
    // TODO
    warnings: Option<Vec<Value>>,
}
