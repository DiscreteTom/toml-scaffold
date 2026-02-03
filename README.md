# toml-template

Generate commented TOML configuration template files from Rust structs and values.

## Features

- Preserve doc comments as TOML comments
- Support for common types: primitives, Option, HashMap, Vec, nested structs and `serde_json::Value`

## Installation

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
schemars = "1"
toml-template = "0.1"
```

## Usage

```rust
use serde::{Serialize};
use schemars::JsonSchema;
use toml_template::TomlTemplate;

#[derive(Serialize, TomlTemplate, JsonSchema)]
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
    let template = config.to_template().unwrap();
    println!("{}", template);
}
```

Output:

```toml
# Server host address
host = "localhost"
# Server port
port = 8080
```
