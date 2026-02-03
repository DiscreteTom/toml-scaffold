# toml-scaffold

[![Crates.io](https://img.shields.io/crates/v/toml-scaffold)](https://crates.io/crates/toml-scaffold)
[![License](https://img.shields.io/github/license/DiscreteTom/toml-scaffold)](https://github.com/DiscreteTom/toml-scaffold/blob/main/LICENSE)

Generate commented TOML configuration scaffold files from Rust structs and values.

## Features

- Preserve doc comments as TOML comments
- Preserve field order in generated TOML
- Support for common types: primitives, `Option`, `HashMap`, `Vec`, nested structs and `serde_json::Value`

## Installation

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
schemars = "1"
toml-scaffold = "0.1"
```

## Usage

```rust
use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

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
# Server host address
host = "localhost"
# Server port
port = 8080
```

## [More Examples](./crates/toml-scaffold/tests/)

## [CHANGELOG](./CHANGELOG.md)
