# serde-tuplex

tuple serialization and lenient numeric parsing for [serde](https://crates.io/crates/serde)

## Features

- **Lenient parsing**: Numeric fields accept both numbers and strings (`123` or `"123"`)
- **Tuple format**: Serialize structs as arrays (`[1, 2]` instead of `{"a": 1, "b": 2}`)
- **Zero boilerplate**: Just add `#[derive(Lenient)]`
- **Field-level control**: Use `#[serde_tuplex(skip)]` or `lenient` helper

## Installation

```toml
[dependencies]
serde-tuplex = "0.1"
serde = { version = "1.0", features = ["derive"] }
```

## Usage

### Lenient (struct format)

```rust
use serde_tuplex::Lenient;

#[derive(Lenient)]
struct User {
    id: u64,           // accepts 100 or "100"
    balance: f64,      // accepts 99.99 or "99.99"
    age: Option<u32>,  // accepts numbers, strings, null, or missing
    name: String,      // unchanged
}
```

### TupleLenient (tuple format + lenient parsing)

```rust
use serde_tuplex::TupleLenient;

#[derive(TupleLenient)]
struct Measurement {
    timestamp: u64,
    value: f64,
}

// Accepts: [1234567890, 23.5] or ["1234567890", "23.5"]
// Serializes to: [1234567890, 23.5]
```

### Tuple (tuple format, strict parsing)

```rust
use serde_tuplex::Tuple;

#[derive(Tuple)]
struct Point {
    x: u64,
    y: u64,
}

// Serializes to: [10, 20]
```

### With raw serde

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    #[serde(deserialize_with = "serde_tuplex::lenient")]
    count: u64,        // Lenient

    strict: u32,       // Strict
}
```

### Field-level control

```rust
#[derive(Lenient)]
struct Config {
    timeout: u64,      // Auto: lenient

    #[serde_tuplex(skip)]
    port: u16,         // Strict parsing only
}
```
