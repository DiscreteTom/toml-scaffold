# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **BREAKING**: Scalar arrays now always format inline regardless of element count. Previously, arrays with 5+ elements were formatted as multiline. Users can use `#[format = "multiline"]` to force multiline format when needed.
- **BREAKING**: `TomlScaffold::format_preferences()` now returns `HashMap<FieldPath, String>` instead of `HashMap<String, String>`. `FieldPath` is now public.

### Fixed

- `#[format]` attributes on nested struct fields now work correctly. Format preferences are recursively collected from all nested types.

## [0.3.0] - 2026-02-18

### Added

- `#[format]` attribute for customizable field formatting
  - `"inline"` - Render nested structs as inline tables: `{ key = value }`
  - `"dotted"` - Flatten one level with dotted keys: `field.key = value`
  - `"dotted-nested"` - Recursively flatten all levels: `field.key.subkey = value`
  - `"*dotted"` - Keep `[section]` header, flatten children with dotted keys
  - `"*dotted-nested"` - Keep `[section]` header, recursively flatten all children
  - `"multiline"` - Force multiline array format
- Support for `#[format]` attribute on `serde_json::Value` fields

## [0.2.1] - 2026-02-09

### Fixed

- Prevent duplicate empty lines before TOML sections

## [0.2.0] - 2026-02-08

### Added

- Support for root struct doc comments - prepended to TOML output with blank line separator

### Changed

- Empty lines in multiline doc comments are automatically removed (matches schemars behavior)

### Fixed

- Handle field names containing dots correctly
- Properly escape TOML keys with special characters (dots, spaces, etc.) using `toml_writer` crate
- Use `toml_writer` for proper string escaping and formatting

## [0.1.0] - 2026-02-03

### Added

- Initial release
- Generate commented TOML configuration scaffold files from Rust structs
- Preserve doc comments as TOML comments
- Support for primitives, Option, HashMap, Vec, nested structs and `serde_json::Value`
- Enable `preserve_order` feature for toml dependency to maintain field order

[unreleased]: https://github.com/DiscreteTom/toml-scaffold/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/DiscreteTom/toml-scaffold/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/DiscreteTom/toml-scaffold/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/DiscreteTom/toml-scaffold/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/DiscreteTom/toml-scaffold/releases/tag/v0.1.0
