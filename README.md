# toml-scaffold

[![Crates.io](https://img.shields.io/crates/v/toml-scaffold)](https://crates.io/crates/toml-scaffold)
[![License](https://img.shields.io/github/license/DiscreteTom/toml-scaffold)](https://github.com/DiscreteTom/toml-scaffold/blob/main/LICENSE)
[![codecov](https://codecov.io/gh/DiscreteTom/toml-scaffold/branch/main/graph/badge.svg)](https://codecov.io/gh/DiscreteTom/toml-scaffold)

Generate commented TOML configuration scaffold files from Rust structs and values.

## Features

- Preserve doc comments as TOML comments
- Preserve field order in generated TOML
- Support for common types: primitives, `Option`, `HashMap`, `Vec`, nested structs and `serde_json::Value`
- Customizable formatting with `#[format]` attribute

## Installation

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
schemars = "1"
toml-scaffold = "0.2"
```

## Usage

### Basic Example

```rust
use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

/// Server configuration
#[derive(Serialize, JsonSchema, TomlScaffold)]
struct Config {
    /// Server host address
    host: String,
    /// Server port
    port: u16,
}

fn main() {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
    };
    let scaffold = config.to_scaffold().unwrap();
    println!("{}", scaffold);
}
```

Output:

```toml
# Server configuration

# Server host address
host = "localhost"
# Server port
port = 8080
```

### Custom Formatting

Use `#[format = "..."]` to control how fields are rendered:

```rust
use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, JsonSchema, TomlScaffold)]
struct Database {
    host: String,
    port: u16,
}

#[derive(Serialize, JsonSchema, TomlScaffold)]
struct Config {
    name: String,
    
    /// Inline table format
    #[format = "inline"]
    database: Database,
    
    /// Dotted keys format
    #[format = "dotted"]
    server: Database,
    
    /// Child dotted format - keep [section], flatten children
    #[format = "*dotted"]
    settings: serde_json::Value,
}
```

Output:

```toml
name = "myapp"
database = { host = "localhost", port = 5432 }
server.host = "0.0.0.0"
server.port = 8080

[settings]
key1 = 123
nested.key2 = 456
```

**Format Options:**

- `"inline"` - Inline table: `{ key = value }`
- `"dotted"` - Flatten one level: `field.key = value`
- `"dotted-nested"` - Recursively flatten: `field.key.subkey = value`
- `"*dotted"` - Keep `[field]` section, flatten children
- `"*dotted-nested"` - Keep `[field]` section, recursively flatten children
- `"multiline"` - Force multiline array format

## [More Examples](./crates/toml-scaffold/tests/)

## [CHANGELOG](./CHANGELOG.md)
