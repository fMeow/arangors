# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.5.0](https://github.com/fMeow/arangors/compare/v0.4.8...v0.5.0) (2021-05-01)


### Features

* **deps:** add rustls features for reqwest ([90efc95](https://github.com/fMeow/arangors/commit/90efc95e3a73928652504bea85cfd3fa638995a0))
* **deps:** update reqwest to 0.11, tokio to 1 ([a4c3c95](https://github.com/fMeow/arangors/commit/a4c3c950d991055f4c40d7a7960eb2ec0d867eca))
* use uclient for http request ([b03312b](https://github.com/fMeow/arangors/commit/b03312b68ef5899665f67825da1cb5e519d4a7f3))


### Bug Fixes

* use uclient instead of arangors::client ([823e6e1](https://github.com/fMeow/arangors/commit/823e6e1b10797a09ef36a85ca25e7d5c18169ac9))

### [0.4.8](https://github.com/fMeow/arangors/compare/v0.4.7...v0.4.8) (2021-05-01)


### Bug Fixes

* **deps:** revert reqwest to 0.10 and tokio to 0.2 ([fea6a81](https://github.com/fMeow/arangors/commit/fea6a81745cc2b2ddd895e2a0cf01627c0e01c58))

### [0.4.7](https://github.com/fMeow/arangors/compare/v0.4.6...v0.4.7) (2021-03-28)


### ⚠ BREAKING CHANGES

* **client:** rename copy_with_transaction to clone_with_transaction
* NgramAnalyzerProperties.preserve_riginal is renamed to preserve_original

### Features

* **client:** rename copy_with_transaction ([2c81c78](https://github.com/fMeow/arangors/commit/2c81c78f935f605df33792bce2f6455117290686))
* Add Debug derive on same options structs ([10c5265](https://github.com/fMeow/arangors/commit/10c526528c4e9a31ba1879eaf14db6b7c9d35bfb))
* **collection:** clone_with_transaction ([79379f5](https://github.com/fMeow/arangors/commit/79379f5f432366e98469534201d0a0afd9c428ba))


### Bug Fixes

* catch error for wrong credentials for connection([#69](https://github.com/fMeow/arangors/issues/69)) ([f8c3303](https://github.com/fMeow/arangors/commit/f8c3303b8dfed894ab1939ac5eab8a4c3cbf2780))
* disable openssl in example reqwest_rustls ([375ecf4](https://github.com/fMeow/arangors/commit/375ecf48418d648ecbfd855f0acda6d36022f205))
* enable multi-thread feature on tokio ([56e17d5](https://github.com/fMeow/arangors/commit/56e17d52b1f6e46641fefc2cf04c5a4fcaa792b9))
* fix typo in 'NgramAnalyzerProperties' ([#61](https://github.com/fMeow/arangors/issues/61)) ([361f31b](https://github.com/fMeow/arangors/commit/361f31ba9912dda7a82d3832d6709b4e42a6c8a7))
* rename copy_with_transaction to clone_with_transaction ([bda9457](https://github.com/fMeow/arangors/commit/bda9457d156b0e9dd394adcb1633e75c9469f3f2))

### [0.4.6](https://github.com/fMeow/arangors/compare/v0.4.5...v0.4.6) (2021-01-27)


### Features

* Graph Options correction, Clone Implementation on public structs ([#51](https://github.com/fMeow/arangors/issues/51)) ([a669281](https://github.com/fMeow/arangors/commit/a66928112d8c022a7fb5f68aec872db4edcd8f7a))
* Support for transactions, analyzers and views ([#38](https://github.com/fMeow/arangors/issues/38)) ([1be43eb](https://github.com/fMeow/arangors/commit/1be43ebef82a66ff1f203845b279a1ac8907da67))

### [0.4.5](https://github.com/guoli-lyu/arangors/compare/v0.4.4...v0.4.5) (2020-11-26)


### Features

* Graph Management ([#47](https://github.com/guoli-lyu/arangors/issues/47)) ([c9b4a53](https://github.com/guoli-lyu/arangors/commit/c9b4a53f2f88fa8225b7c11d0e044deca798aeb0))


### Bug Fixes

* use Error type instead of unwrap for Doc deser ([4d41a71](https://github.com/guoli-lyu/arangors/commit/4d41a71050ced7747b2485661796f30c4132a37d))

### [0.4.4](https://github.com/guoli-lyu/arangors/compare/v0.4.3...v0.4.4) (2020-11-15)


### ⚠ BREAKING CHANGES

* use DeserializeOwned instead of Deserialize<'de> for Document. This should be alright.

### Features

* add AsRef and Deref for Document ([7f19ccf](https://github.com/guoli-lyu/arangors/commit/7f19ccff9779e77ed860b6a86b5a11a0b9812fa7))
* custom deser for Document allow header in user struct ([fd2c47d](https://github.com/guoli-lyu/arangors/commit/fd2c47d6c8ded83bf58a318854c8083008297aa4))


### Bug Fixes

* breaking API in surf 2.0.0-alpha5 ([349cd16](https://github.com/guoli-lyu/arangors/commit/349cd1679a582796966bae5c9e9e46d4e57f9663))
* change all info level log to debug ([#34](https://github.com/guoli-lyu/arangors/issues/34)) ([cff0653](https://github.com/guoli-lyu/arangors/commit/cff06530e0038010c07e03bff4a2d59b253a3cff))
* Fix bug in fetch-all ([#42](https://github.com/guoli-lyu/arangors/issues/42)) ([cad3923](https://github.com/guoli-lyu/arangors/commit/cad392365b84d86dd7041f220284356f3679a3d2))
* Remove unnecessary mutual borrow ([#39](https://github.com/guoli-lyu/arangors/issues/39)) ([148af62](https://github.com/guoli-lyu/arangors/commit/148af62e2952f5873f35428b065735b5ae41df63))

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
