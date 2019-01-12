# arangors

[![Build Status](https://travis-ci.org/Guoli-Lyu/arangors.svg?branch=master)](https://travis-ci.org/Guoli-Lyu/arangors)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/arangors.svg)](https://crates.io/crates/arangors)
[![arangors](https://docs.rs/arangors/badge.svg)](https://docs.rs/arangors)

`arangors` is an intuitive rust client for [arangoDB](https://www.arangodb.com/),
inspired by [pyArango](https://github.com/tariqdaouda/pyArango).

`arangors` enables you to connect with arangoDB server, access to database,
execute AQL query, manage arangoDB in an easy and intuitive way.

## NOTICE

`arangors` is targeted at `Rust 2018`. Currently only `nightly` channel is supported 
as `arangors` use a unstable feature for external doc.
And this feature will be removed in the near future to make this crate available in stable channel.

Also, `arangors` will stay **synchronous** until the `futures` crate reach
`1.0`, which means stable API for async/await.

## Philosophy of arangors

`arangors` is targeted at ergonomic, intuitive and OOP alike API for arangoDB,
both top level and low level.

Overall architecture of arangoDB:

> databases -> collections -> collections -> documents/edges

And we would like the structure of our top level of client object just like
this.

In fact, the design of `arangors` just mimic this architecture, with a
slight difference that in the top level, there is a connection object on top
of databases. The connection object contains a HTTP client with
authentication information in HTTP headers, so I would rather call a client
session.

## Features

By now, the available features of arangors are:

- make connection with arangors
- get list of databases and collections
- full features AQL query

## TODO

- (Done) Milestone 0.1.x

Synchronous connection based on `reqwest` and full features AQL query.

- (WIP) Milestone 0.2.x

Fill the unimplemented API in `Connection`, `Database`, `Collection` and `Document`.

In this stage, all operations available for database, collection and document should be implemented.

- Milestone 0.3.x

Provides the API related to graph, index and user management.

- Milestone 0.4.x

Abandon `reqwest` and implement asynchronous version using `hyper`.
And the synchronous version is just a wrapper around async version just like the design of `reqwest`.

## Glance

### Connection

There is three way to establish connections:

- jwt
- basic auth
- no authentication

So are the `arangors` API.

When a connection is successfully established,
`arangors` will automatically fetch the structure of arangoDB
by get the list of database, and then lists of collections per database.

Example:

```rust,ignore
use arangors::Connection;

// (Recommended) Handy functions
let conn = Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
let conn =
    Connection::establish_basic_auth("http://localhost:8529", "username", "password").unwrap();
let conn = Connection::establish_without_auth("http://localhost:8529")
    .unwrap();
```

### Database && Collection

```rust, ignore
use arangors::Connection;

fn main(){
    let conn = Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
    let db = conn.get_database("_system").unwrap();
    let collection = db.get_collection("_apps").unwrap();
}
```

### AQL Query

All aql query related functions are associated with database, as AQL query
is performed at database level.

There are several way to execute AQL query, and can be categorized into two
classes:

- batch query

  - `aql_query_batch`
  - `aql_next_batch`

- query to fetch all result
  - `aql_str`
  - `aql_bind_vars`
  - `aql_query`

This later category provides a convenient high level API, whereas batch
query offers more control.

Note that results can be strong typed given deserializable struct, or
arbitrary JSON object with `serde::Value`.

- Arbitrary JSON object

```rust, ignore
let resp: Vec<Value> = database
    .aql_str("FOR u IN Collection LIMIT 3 RETURN u")
    .unwrap();
```

- Strong typed result

```rust, ignore
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

let resp: Vec<User> = database.aql_str("FOR u IN users RETURN u").unwrap();
```

Example:

Users have to construct a `AqlQuery` object first. And `AqlQuery` offer all
the options needed to tweak AQL query. You can set batch size, add bind
vars, limit memory, and all others
options available.

```rust,ignore
use arangors::{AqlQuery, Connection, Cursor, Database};
use serde_json::value::Value;

fn main() {
    let conn =
        Connection::establish_jwt("http://localhost:8529", "username", "password").unwrap();
    let database = conn.get_database("database").unwrap();

    let aql = AqlQuery::new("FOR u IN @@collection LIMIT 3 RETURN u").batch_size(1).count(true).bind_var("collection","test_collection");

    let resp: Vec<Value> = database.aql_query(aql).unwrap();
    println!("{:?}", resp);
}
```

Strong typed Query result with `aql_str`:

```rust, ignore
use serde_derive::Deserialize;
#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub password: String,
}

fn main() {
    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();
    let db = conn.get_database("test_db").unwrap();
    let result: Vec<User> = db
        .aql_str(r#"FOR i in test_collection FILTER i.username=="test2" return i"#)
        .unwrap();
}
```

### Contributing

Contributions and feed back are welcome following Github workflow.

### License

`arangors` is provided under the MIT license. See [LICENSE](./LICENSE).
