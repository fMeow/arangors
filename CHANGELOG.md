# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.4.3](https://github.com/fMeow/arangors/compare/v0.4.2...v0.4.3) (2020-08-20)


### ⚠ BREAKING CHANGES

* **Connection:** validate_server is now a static method
* rename r#type field of
collection::response::Info to collection_type

### Features

* index management ([#33](https://github.com/fMeow/arangors/issues/33)) ([b2c4234](https://github.com/fMeow/arangors/commit/b2c423443e7c8f1db6d4d778515018397f4d3806))
* **Connection:** validate_server is now static method ([e908d47](https://github.com/fMeow/arangors/commit/e908d473605af6a835076892742f722b17d5260c))
* **database:** add method to get database name ([fa7a409](https://github.com/fMeow/arangors/commit/fa7a409ecc2081ac4682e77c1f04cd86a0ff0928))
* get db struct from a collection ([20d2505](https://github.com/fMeow/arangors/commit/20d25053feeea24ec916c5ddf9495a17396ba5a1))


### Bug Fixes

* rename collection_type to type when deserialize ([e99a8d5](https://github.com/fMeow/arangors/commit/e99a8d50df04822136598dfd5824ff09985c1a0d))


* rename r#type to collection_type ([2bbfe19](https://github.com/fMeow/arangors/commit/2bbfe1980130519931a809be6ead0989902cf34d))

## [0.4.2](https://github.com/fMeow/arangors/compare/v0.4.1...v0.4.2) (2020-07-26)


### ⚠ BREAKING CHANGES

* return CollectionType instead of reference
for Collection::collection_type()
* Removes the phantom lifetime field from Database and Collection.

### Features

* Add QueryBuilder::try_bind ([#25](https://github.com/fMeow/arangors/issues/25)) ([bbe2941](https://github.com/fMeow/arangors/commit/bbe2941f0843b0af954c3e6466562134d3a98904))
* Remove lifetimes from Database and Collection ([#23](https://github.com/fMeow/arangors/issues/23)) ([222445e](https://github.com/fMeow/arangors/commit/222445ef4aa893a0b7a3894ab502d203e5331363))
* return CollectionType instead of a reference ([b46c832](https://github.com/fMeow/arangors/commit/b46c83217bda52a4ce8a601ac837acda962f4795))
