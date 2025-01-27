# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.4.2](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.1...mrml-wasm-v1.4.2) - 2024-01-09

### Other
- *(mrml-wasm)* update package building ([#362](https://github.com/jdrouet/mrml/pull/362))

## [1.4.1](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.4.0...mrml-wasm-v1.4.1) - 2023-12-20

### Other
- release ([#360](https://github.com/jdrouet/mrml/pull/360))

## [1.4.0](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.3.0...mrml-wasm-v1.4.0) - 2023-12-07

### Added
- *(mrml-core)* make the include_loader be Send and Sync ([#347](https://github.com/jdrouet/mrml/pull/347))

### Other
- *(mrml-wasm)* update Cargo.toml
- *(mrml-core)* rename Options to RenderOptions ([#352](https://github.com/jdrouet/mrml/pull/352))
- update github actions configuration ([#351](https://github.com/jdrouet/mrml/pull/351))
- release ([#348](https://github.com/jdrouet/mrml/pull/348))
- *(mrml-core)* release version 2.0.0

## [1.3.0](https://github.com/jdrouet/mrml/compare/mrml-wasm-v1.2.3...mrml-wasm-v1.3.0) - 2023-12-01

### Added
- handle properly async mj-include ([#346](https://github.com/jdrouet/mrml/pull/346))
- implement an async parser ([#338](https://github.com/jdrouet/mrml/pull/338))
- *(mrml-wasm)* add mj-include feature ([#316](https://github.com/jdrouet/mrml/pull/316))
- *(mrml-wasm)* create wasm package ([#312](https://github.com/jdrouet/mrml/pull/312))
- *(wasm)* add toJson, toMjml and validate

### Fixed
- *(mrml-wasm)* disable_comments is in camelCase
- *(json)* add serde to mrml-wasm

### Other
- bump mrml-core to 2.0.0-rc.7
- bump mrml-core to 2.0.0-rc.6
- bump deps ([#328](https://github.com/jdrouet/mrml/pull/328))
- *(deps)* Update serde-wasm-bindgen requirement from 0.5 to 0.6 in /packages/mrml-wasm ([#327](https://github.com/jdrouet/mrml/pull/327))
- *(mrml-core)* bump to 2.0.0-rc4
- *(mrml-core)* use a single MrmlParser structure with a visitor pattern ([#317](https://github.com/jdrouet/mrml/pull/317))
- format code using rustfmt and create config
- *(mrml-wasm)* add test with disabling comments
- *(mrml-wasm)* update node example to have test command
- *(mrml-wasm)* add browser example ([#315](https://github.com/jdrouet/mrml/pull/315))
- *(mrml-wasm)* add node example with a test ([#313](https://github.com/jdrouet/mrml/pull/313))
- *(mrml-wasm)* update readme
- move code source to only keep mrml-core
- apply clippy suggestions
- *(mrml-wasm)* release version 1.2.3
- *(mrml-wasm)* release version 1.2.2
- *(mrml-wasm)* release version 1.2.1
- release version 1.2.0
- release version 1.1.0
- rename packages folder
