# ephais-error

A minimal shared error-handling crate for the Ephais ecosystem.

## Overview

This crate exposes a single `Error` struct—plus a `Severity` enum—to unify how errors are represented across multiple Rust projects in the Ephais ecosystem. You can specify:

- **Severity** (`Critical`, `Error`, `Warning`, `Info`)
- **Reference code** (e.g., "NET-001", "FSY-404")
- **Human-readable message**
- **Optional metadata** for extended context
- **Optional underlying error source** to chain other errors

By keeping it simple, you avoid clutter and frequent version bumps in a large shared error type.

## Map Ephais Err

### Why
. Mapping errors manually with map_err can lead to repetitive code and potential inconsistencies throughout your projects. By providing an extension trait, you standardize error conversion across your ecosystem, making your code cleaner and error messages uniform.

### What
The EphErrorExt trait adds a method (map_ephais_err) to Result<T, E>. This method converts any error implementing std::error::Error into your defined Error type. In doing so, it attaches:

A defined severity (Severity)
An error reference code (e.g., "FSY-404")
A detailed human-readable description that includes the original error message
The underlying error itself (as a source)

### How
Import the Trait:

```rust
use ephais_error::{Severity, Result};
use ephais_error::EphErrorExt;
```

In your consumer crates, import the trait along with the required types:

Convert Errors Using the Method:

```rust
use std::fs::File;

let file = File::open("path/to/file")
    .map_ephais_err(Severity::Error, "FSY-1", "Can't open file")?;
```

Instead of applying map_err manually, call map_ephais_err when handling results:

Benefits:

* Cleaner Code: Reduce boilerplate by encapsulating error conversion into a single method.
* Consistency: Ensures all parts of your project use a unified approach for error handling.
* Enhanced Debugging: Automatically chains the source error, preserving the original error context for easier troubleshooting.

By adopting this approach, you'll maintain a concise and consistent error handling pattern across your applications.

## Old Usage

### 1. Add as a dependency

In your `Cargo.toml`:

```toml
[dependencies]
ephais-error = { git = "ssh://git@github.com/ephais/ephais-error.git", tag = "v0.1.0" }
```

(Or reference your local path / desired branch.)

### 2. Creating a basic error

```rust
use ephais_error::{Error, Severity};

fn my_func() -> ephais_error::Result<()> {
    // Creating a basic error without a source
    let err = Error::new(Severity::Error, "NET-001", "Connection timed out");
    Err(err)
}
```

### 3. Adding a source error

If you have an `std::io::Error` or another error that implements `std::error::Error`, attach it to `Error`:

```rust
use ephais_error::{Error, Severity, Result};
use std::io;

fn read_file() -> Result<String> {
    let content = std::fs::read_to_string("myfile.txt")
        .map_err(|io_err| {
            Error::new(Severity::Error, "FSY-404", "Cannot read file")
                .with_source(Box::new(io_err))
        })?;
    Ok(content)
}
```

When you print or log `Error`, you’ll see both the main description **and** the source error message.

### 4. Metadata

Store additional context in the `metadata` field:

```rust
let mut err = Error::new(Severity::Warning, "PARSE-100", "Invalid format");
err = err.insert_metadata("filename", "data.json");
err = err.insert_metadata("line", "42");

println!("{}", err);
// [WARN] Ref: PARSE-100 | Invalid format
// (the metadata isn't shown by default, but can be useful for debugging or logging)
```

## Why Use ephais-error?

- **Simplicity**: One flexible error type means fewer crates or enum variants to maintain.
- **Consistency**: All errors share the same fields for severity, reference codes, and messages.
- **Extensibility**: Crates can add domain-specific references, attach any error source, or store arbitrary metadata—without needing to change this crate.

## Contributing

Feel free to open pull requests or issues in the Ephais organization repository. The goal is to keep `ephais-error` small, so carefully consider if your feature truly needs to live here instead of your own crate.

## License

This crate is proprietary to the Ephais ecosystem (or add your preferred license statement here).

