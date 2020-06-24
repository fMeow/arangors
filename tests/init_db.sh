#!/bin/bash

# Run the scrpit as follows from your terminal before starting 'cargo test':
# . ./init_db.sh

export ARANGODB_HOST=${ARANGODB_HOST:="localhost:8529/"}
export ARANGO_ROOT_USER=${ARANGO_ROOT_USER:="root"}
export ARANGO_ROOT_PASSWORD=${ARANGO_ROOT_PASSWORD:="KWNngteTps7XjrNv"}
export ARANGO_USER=${ARANGO_USER:="username"}
export ARANGO_PASSWORD=${ARANGO_PASSWORD:="password"}

# set up database
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_api/database" --data '{"name":"test_db"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/collection" --data '{"name":"test_collection"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/collection" --data '{"name":"test_collection1"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/collection" --data '{"name":"test_collection2"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/collection" --data '{"name":"test_collection3"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/collection" --data '{"name":"test_collection4"}'
curl -X DELETE "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/collection/test_collection4"

# set up normal user
curl -X DELETE "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_api/user/username"
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_api/user" --data '{"user":"username","passwd":"password"}'
curl -X PUT  "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_api/user/username/database/test_db" --data '{"grant":"rw"}'
curl -X PUT  "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_api/user/username/database/test_db/test_collection1" --data '{"grant":"rw"}'
curl -X PUT  "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_api/user/username/database/test_db/test_collection2" --data '{"grant":"ro"}'
curl -X PUT  "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_api/user/username/database/test_db/test_collection3" --data '{"grant":"none"}'

# set up collection
curl -X PUT "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/collection/test_collection/truncate"
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test","password":"test_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test2","password":"test2_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test3","password":"test3_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test4","password":"test4_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test5","password":"test5_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test6","password":"test6_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test7","password":"test7_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test8","password":"test8_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test9","password":"test9_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test10","password":"test10_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test11","password":"test11_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test12","password":"test12_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test13","password":"test13_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test14","password":"test14_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test15","password":"test15_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test16","password":"test16_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test17","password":"test17_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test18","password":"test18_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test19","password":"test19_pwd"}'
curl -X POST "http://${ARANGO_ROOT_USER}:${ARANGO_ROOT_PASSWORD}@${ARANGODB_HOST}_db/test_db/_api/document/test_collection" --data '{"username":"test20","password":"test20_pwd"}'
