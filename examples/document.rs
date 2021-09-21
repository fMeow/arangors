#![allow(unused_imports)]
#![allow(unused_parens)]

use anyhow::Error;

use arangors::{document::options::InsertOptions, Collection, Connection};

use arangors::document::{
    options::{RemoveOptions, ReplaceOptions, UpdateOptions},
    response::DocumentResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

const URL: &str = "http://localhost:8529";

#[derive(Serialize, Deserialize, Debug)]
struct User {
    first_name: String,
    last_name: String,
    email: String,
}

#[cfg_attr(feature = "reqwest_async", tokio::main)]
#[cfg_attr(feature = "surf_async", async_std::main)]
#[cfg_attr(feature = "blocking", maybe_async::must_be_sync)]
async fn main() -> Result<(), Error> {
    let collection_name = "test_collection_document_example";

    let conn = Connection::establish_jwt(URL, "username", "password").await?;
    let database = conn.db("test_db").await?;
    database.create_collection(collection_name).await.unwrap();
    let collection = database.collection(collection_name).await?;

    let new_user = User {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@who".to_string(),
    };

    // create a document
    let new_doc_response = collection
        .create_document(new_user, InsertOptions::builder().return_new(true).build())
        .await
        .unwrap();

    let new_doc = new_doc_response.new_doc();
    eprintln!(
        "Your new document should have been created -> {:?} ",
        new_doc
    );

    // update a document
    let header = new_doc_response.header().unwrap();
    let _key = &header._key;

    let patch = json!({"last_name" : "Doh"});
    let update_doc_response = collection
        .update_document(
            _key,
            patch,
            UpdateOptions::builder()
                .return_new(true)
                .return_old(true)
                .build(),
        )
        .await
        .unwrap();

    let new_doc = update_doc_response.new_doc();
    eprintln!("John Doe is now John Doh -> {:?}", new_doc);

    let old_doc = update_doc_response.old_doc();
    eprintln!("John Doh was called John Doe before ->  {:?}", old_doc);

    // update a document with default option
    let header = update_doc_response.header().unwrap();
    let old_rev = &header._rev;

    let patch = json!({"email" : "john.doh@who"});
    // use Default::default() to set default options
    let update_doc_response = collection
        .update_document(_key, patch, Default::default())
        .await
        .unwrap();

    let header = update_doc_response.header().unwrap();
    let _rev = &header._rev;

    if old_rev != _rev {
        eprintln!("John Doh has changed his address email");
    }

    // replace a document
    let replace = User {
        first_name: "Bob".to_string(),
        last_name: "Johnson".to_string(),
        email: "bob.Johnson@internet".to_string(),
    };

    let replace_doc_response = collection
        .replace_document(
            _key,
            replace,
            ReplaceOptions::builder()
                .return_new(true)
                .return_old(true)
                .build(),
            Some(_rev.to_string()),
        )
        .await
        .unwrap();

    let new_doc = replace_doc_response.new_doc();
    eprintln!(
        "John Doh found his identity, his real name is Bob Johnson with email \
         bob.Johnson@internet@-> {:?}",
        new_doc
    );

    let old_doc = replace_doc_response.old_doc();
    eprintln!(
        "Bob Johnson was called John Doh because he did not remember who he was ->  {:?}",
        old_doc
    );

    // remove a document
    let remove_doc_response: DocumentResponse<User> = collection
        .remove_document(
            _key,
            RemoveOptions::builder().return_old(true).build(),
            None,
        )
        .await
        .unwrap();

    let old_doc = remove_doc_response.old_doc();
    eprintln!(
        "Bob Johnson has been removed from the Database which helps people to remember their \
         identity ->  {:?}",
        old_doc
    );

    // cleanup
    database.drop_collection(collection_name).await.unwrap();

    Ok(())
}
