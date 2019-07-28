use arangors::{AqlQuery, Connection, Cursor, Database};
use serde_json::value::Value;
const URL: &str = "http://localhost:8529";

fn main() {
    env_logger::init();

    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv").unwrap();

    let database = conn.db("test_db").unwrap();
    let aql = AqlQuery::new("FOR u IN test_collection LIMIT 3 RETURN u");
    println!("{:?}", aql);
    println!("{:?}", serde_json::to_string(&aql).unwrap());

    let resp: Vec<Value> = database.aql_query(aql).unwrap();
    println!("{:?}", resp);
}
