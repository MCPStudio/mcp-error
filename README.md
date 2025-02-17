# ephais-error

A **central error-handling crate** for the Ephais ecosystem, providing:

- **Detailed error context** (severity, reference codes, metadata)
- **Easy creation** of domain-specific error variants
- **Unified, consistent** error formatting across multiple projects

## Why does this crate exist?

Most Rust libraries define their own error type, but in a large organization or ecosystem, you often need to:

1. **Standardize** how errors are represented, so that logs and error reporting are consistent.
2. Include **metadata** (e.g., references, severity) to handle errors differently based on severity, or track them with unique codes.
3. Allow advanced composition of errors (e.g., wrapping an I/O error and adding domain context).

`ephais-error` handles all that in one place, so you can:

- Reuse the same definitions in different crates or microservices.
- Expand or refine your error variants in one shared location.

## How this crate works

1. **ErrorContext** – the struct that holds useful information about an error:

   - `reference`: A short code or name for the error (e.g., `NET-TIMEOUT`)
   - `severity`: The error’s severity level (`CRIT`, `ERR`, `WARN`, `INFO`)
   - `description`: A human-readable description, like "Failed to connect to server"
   - `metadata`: Additional info stored in a `HashMap<String, String>` (optional)

2. **Severity** – an enum listing different severities: `Critical`, `Error`, `Warning`, and `Info`.

3. **Error** – the main error enum, with variants for different categories:

   - `Network(ErrorContext)`
   - `DataFormat(ErrorContext)`
   - `Unknown(ErrorContext)`
   - `FileSystem(ErrorContext, Box<dyn std::error::Error + Send + Sync>)`
   - `External(ErrorContext, Box<dyn std::error::Error + Send + Sync>)`

   FileSystem and External errors allow a **source error** to be embedded (e.g., an underlying I/O error), making it easy to track the root cause.

4. \*\*No \*\***`thiserror`** – We manually implement `std::fmt::Display` and `std::error::Error` instead of using a macro. This is a design choice that gives us full control and avoids extra dependencies.

## Usage

### 1. Add it to your Cargo.toml

```toml
[dependencies]
ephais-error = { git = "ssh://git@github.com/ephais/ephais-error.git", tag = "v0.1.1" }
```

(or point to your preferred branch, commit, or local path)

### 2. Use `Error` or `Result` in your code

```rust
use ephais_error::{Error, Result};

fn do_something() -> Result<()> {
    // For example, create a network error
    Err(Error::network("TIMEOUT", "Connection timed out"))
}
```

If you need an error that references an I/O or external error:

```rust
use ephais_error::{Error, Result};

fn read_file() -> Result<String> {
    let content = std::fs::read_to_string("somefile.txt")
        .map_err(|io_err| Error::file_system("READFAIL", "Could not read file", Box::new(io_err)))?;
    Ok(content)
}
```

### 3. Interpreting errors

When you print or log an `Error`, you get a standardized format, for example:

```
[NETWORK] ERR | Ref: NET-TIMEOUT | Connection timed out
```

You can also check `source()` if you need the underlying cause:

```rust
fn handle_error(err: ephais_error::Error) {
    println!("Encountered error: {}", err);
    if let Some(src) = err.source() {
        println!("Caused by: {}", src);
    }
}
```

## Extending or customizing

- If you want new error variants (e.g., `AuthError`), add another variant to the `Error` enum and create a constructor method.
- If you need extra fields (like a user ID or request ID), place them in `ErrorContext` or store them in `metadata`.

## When to use this crate

- **Multiple projects** in the Ephais ecosystem rely on consistent error types.
- You want to handle or log your errors the same way across many crates.
- You plan to store or forward these errors in a centralized logging system or an error collector.

If you only have a single small crate with minimal errors, local definitions might be simpler. But if you foresee repeated patterns and a need to unify logs, `ephais-error` is a good solution.

## License

This crate is proprietary to the Ephais ecosystem.

