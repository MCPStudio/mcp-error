# ephais-error

A minimal shared error-handling crate for the Ephais ecosystem.

## Overview

This crate exposes a single `EphaisError` struct—plus a `Severity` enum—to unify how errors are represented across multiple Rust projects in the Ephais ecosystem. You can specify:

- **Severity** (`Critical`, `Error`, `Warning`, `Info`)
- **Reference code** (e.g., "NET-001", "FSY-404")
- **Human-readable message**
- **Optional metadata** for extended context
- **Optional underlying error source** to chain other errors

By keeping it simple, you avoid clutter and frequent version bumps in a large shared error type.

## Usage

### 1. Add as a dependency

In your `Cargo.toml`:

```toml
[dependencies]
ephais-error = { git = "ssh://git@github.com/ephais/ephais-error.git", tag = "v0.1.0" }
```

(Or reference your local path / desired branch.)

### 2. Creating a basic error

```rust
use ephais_error::{EphaisError, Severity};

fn my_func() -> ephais_error::Result<()> {
    // Creating a basic error without a source
    let err = EphaisError::new(Severity::Error, "NET-001", "Connection timed out");
    Err(err)
}
```

### 3. Adding a source error

If you have an `std::io::Error` or another error that implements `std::error::Error`, attach it to `EphaisError`:

```rust
use ephais_error::{EphaisError, Severity};
use std::io;

fn read_file() -> ephais_error::Result<String> {
    let content = std::fs::read_to_string("myfile.txt")
        .map_err(|io_err| {
            EphaisError::new(Severity::Error, "FSY-404", "Cannot read file")
                .with_source(Box::new(io_err))
        })?;
    Ok(content)
}
```

When you print or log `EphaisError`, you’ll see both the main description **and** the source error message.

### 4. Metadata

Store additional context in the `metadata` field:

```rust
let mut err = EphaisError::new(Severity::Warning, "PARSE-100", "Invalid format");
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

