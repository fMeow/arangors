# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.4.2](https://github.com/fMeow/arangors/compare/v0.4.1...v0.4.2) (2020-07-26)


### ⚠ BREAKING CHANGES

* return CollectionType instead of reference
for Collection::collection_type()
* Removes the phantom lifetime field from Database and Collection.

### Features

* Add QueryBuilder::try_bind ([#25](https://github.com/fMeow/arangors/issues/25)) ([bbe2941](https://github.com/fMeow/arangors/commit/bbe2941f0843b0af954c3e6466562134d3a98904))
* Remove lifetimes from Database and Collection ([#23](https://github.com/fMeow/arangors/issues/23)) ([222445e](https://github.com/fMeow/arangors/commit/222445ef4aa893a0b7a3894ab502d203e5331363))
* return CollectionType instead of a reference ([b46c832](https://github.com/fMeow/arangors/commit/b46c83217bda52a4ce8a601ac837acda962f4795))
