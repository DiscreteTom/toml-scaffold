# toml-template

Generate commented TOML templates from Rust structs and values.

## Features

- Auto-generate TOML configuration templates from Rust values
- Preserve doc comments as TOML comments
- Support for common types: HashMap, Vec, Option, primitives, nested structs

## Usage

```rust
use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, TomlTemplate)]
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
    let template = config.to_template();
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
