# ephais-error

A **centralized error-handling crate** for the Ephais ecosystem, updated with the latest improvements from lib.rs. It provides:

- **Enhanced error context** (including optional call stacks and extended metadata)
- **Simplified creation** of domain-specific error variants with new helper methods
- **Uniform error formatting** with advanced source tracking

## Why this crate exists

In modern Rust projects, consistent error handling is crucial. In large ecosystems you need to:
1. **Standardize error representation** to ensure uniform logs and reports.
2. Attain **richer metadata support** (including references, severity, and optional call stacks) for improved diagnostics.
3. Facilitate **error composition** by wrapping underlying errors with enhanced context.

`ephais-error` centralizes these concerns and improves over previous iterations by:
- Adding richer context details
- Introducing new variants and helper constructors in lib.rs
- Enhancing integration with custom logging and monitoring systems

## How the crate works

1. **ErrorContext** – now enriched with optional call stack captures and improved metadata:
    - `reference`: A unique error code or name (e.g., `NET-TIMEOUT`)
    - `severity`: The error’s notification level (`CRIT`, `ERR`, `WARN`, `INFO`)
    - `description`: A human-readable explanation (e.g., "Failed to connect to server")
    - `metadata`: Extended key-value context stored in a `HashMap<String, String>`
    - (Optional) `call_stack`: Provides a debug stack trace when enabled

2. **Severity** – an updated enum representing error levels, designed for future extension and custom thresholds.

3. **Error** – a consolidated error enum with improved diagnostics and chaining:
    - `Network(ErrorContext)`
    - `DataFormat(ErrorContext)`
    - `Unknown(ErrorContext)`
    - `FileSystem(ErrorContext, Box<dyn std::error::Error + Send + Sync>)`
    - `External(ErrorContext, Box<dyn std::error::Error + Send + Sync>)`
    - (New) `Auth(ErrorContext)` for authentication-related errors

Variants like FileSystem and External allow embedding source errors for full traceability. We continue to manually implement `Display` and `Error` (avoiding `thiserror`) for complete control and minimal dependencies.

## Usage

### 1. Update your Cargo.toml

```toml
[dependencies]
ephais-error = { git = "ssh://git@github.com/ephais/ephais-error.git", tag = "v0.2.0" }
```

### 2. Use `Error` or `Result` in your code

```rust
use ephais_error::{Error, Result};

fn do_something() -> Result<()> {
     // Creating a network error with extended context
     Err(Error::network("TIMEOUT", "Connection timed out"))
}
```

For errors with an underlying cause:

```rust
use ephais_error::{Error, Result};

fn read_file() -> Result<String> {
     let content = std::fs::read_to_string("somefile.txt")
          .map_err(|io_err| Error::file_system("READFAIL", "Could not read file", Box::new(io_err)))?;
     Ok(content)
}
```

For authentication failures:

```rust
use ephais_error::{Error, Result};

fn authenticate_user() -> Result<()> {
     // Demonstrating the use of the new Auth error variant
     Err(Error::auth("LOGIN_FAIL", "User authentication failed"))
}
```

## Error Handling and Logging

Errors format uniformly. For instance:

```
[NETWORK] ERR | Ref: NET-TIMEOUT | Connection timed out
```

When logging errors, inspect the underlying cause with the `source()` method:

```rust
fn handle_error(err: ephais_error::Error) {
     println!("Encountered error: {}", err);
     if let Some(src) = err.source() {
          println!("Caused by: {}", src);
     }
}
```

## Extending or Customizing

- Extend the `Error` enum to add new variants (e.g., business logic errors).
- Customize error context by enhancing `ErrorContext` or integrating structured logging.
- Use new helper methods in lib.rs for streamlined error creation and composition.

## When to use this crate

- When your projects need a unified error format across multiple components.
- For consistent error logging and centralized diagnostics in the Ephais ecosystem.
- When detailed error context, including metadata and optional call stacks, is required.

## License

This crate is proprietary to the Ephais ecosystem.
