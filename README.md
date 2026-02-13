# common-utils-rs

**common-utils-rs** is a lightweight Rust utility crate providing common, reusable developer utilities.  
The goal of this crate is to reduce boilerplate and provide reliable, easy-to-use tools for Rust projects. Each utility is **feature-gated**, so only what you need is compiled and included.

---

## Features

- **env** – Safe, typed environment variable utilities:
  - Read required variables: `get_required()`
  - Read with default values: `get_or_default()`
  - Parse typed values: `get_parsed()`
  - Boolean helpers: `get_bool()`
  - List helpers (comma-separated): `get_list()`

> All functions are feature-gated under `env` for optional inclusion.

---

## Installation

Add `common-utils-rs` to your `Cargo.toml`:

```toml
[dependencies]
common-utils-rs = { git = "https://github.com/goodies-hub/common-utils-rs.git", features = ["env"] }
```

Enable only the features you need:

```
[dependencies]
common-utils-rs = { git = "https://github.com/goodies-hub/common-utils-rs.git", features = ["env"] }
```

### Usage
```
// Import env utilities (feature-gated)
use common_utils_rs::env::*;

// Get a required environment variable
let username = get_required("USERNAME").expect("USERNAME must be set");

// Get an optional variable with default
let host = get_or_default("HOST", "127.0.0.1");

// Parse numeric or boolean values
let max_clients: usize = get_parsed("MAX_CLIENTS").unwrap_or(100);
let tls_enabled = get_bool("TLS_ENABLED");
```
