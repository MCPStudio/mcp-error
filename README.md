# mcp-error

A minimal shared error-handling crate for the MCP Studio ecosystem.

## Overview

This crate exposes a single `Error` struct—plus a `Severity` enum—to unify how errors are represented across multiple Rust projects in the MCP Studio ecosystem. You can specify:

- **Severity** (`Critical`, `Error`, `Warning`, `Info`)
- **Reference code** (e.g., "NET-001", "FSY-404")
- **Human-readable message**
- **Optional metadata** for extended context
- **Optional underlying error source** to chain other errors

By keeping it simple, you avoid clutter and frequent version bumps in a large shared error type.

## Mapping Errors with MCP Studio Methods

Rather than mapping errors manually with `map_err`, the crate provides an extension trait with three methods to cover different use cases:

- **`.map_mcp_inf`**: Converts the error into an `Error` with Severity set to `Info` and returns a `Result<T>`.
- **`.map_mcp_err`**: Converts the error into an `Error` with Severity set to `Error` and returns a `Result<T>`.
- **`.map_mcp_crit`**: Converts the error into an `Error` with Severity set to `Critical` and returns a `Result<T>`.

For cases where an error is non-recoverable, you can chain the conversion with the `.or_exit()` method, which prints the error and exits the process with code `-1`.

### Why

Mapping errors manually with `map_err` can lead to repetitive code and potential inconsistencies throughout your projects. By providing these extension methods, you standardize error conversion across your ecosystem:
- **Cleaner Code**: Encapsulate error conversion in a single method.
- **Consistency**: Uniform error messages across your projects.
- **Immediate Failure**: Non-recoverable errors can be immediately handled by chaining with `.or_exit()`, terminating the process with a clear, formatted error message.

### What

The `EphErrorExt` trait adds these three methods to `Result<T, E>`:
- `.map_mcp_inf(reference, description) -> Result<T>`
- `.map_mcp_err(reference, description) -> Result<T>`
- `.map_mcp_crit(reference, description) -> Result<T>`

Additionally, the `OrExit` trait adds the `.or_exit()` method to `Result<T, E>`, allowing you to immediately exit the process in case of an error. In other words, for critical error scenarios you can write:

```rust
use std::fs::File;
use mcp_error::{Result, EphErrorExt, OrExit};

// On failure, converts error with Severity::Error and exits if an error occurs.
let file = File::open("path/to/file")
    .map_mcp_err("FSY-ERR", format!("Can't open file '{}'", "path/to/file"))
    .or_exit();
```

### How

1. **Import the Traits**

   In your consumer crates, import the traits along with the required types:

   ```rust
   use mcp_error::{Result};
   use mcp_error::{EphErrorExt, OrExit};
   ```

2. **Convert Errors and Exit When Needed**

   - **For Information-level errors (recoverable)**

     ```rust
     use std::fs::File;
     
     // Continues execution by propagating a converted error
     let file = File::open("path/to/file")
         .map_mcp_inf("FSY-INFO", format!("Can't open file '{}'", "path/to/file"))?;
     ```
     
   - **For Error-level failures (exit on error)**

     ```rust
     use std::fs::File;
     
     // On failure: converts the error with Severity::Error and, by chaining `.or_exit()`,
     // it prints the error and exits with code -1.
     let file = File::open("path/to/file")
         .map_mcp_err("FSY-ERR", format!("Can't open file '{}'", "path/to/file"))
         .or_exit();
     ```

   - **For Critical-level failures (exit on error)**

     ```rust
     use std::fs::File;
     
     // On failure: converts the error with Severity::Critical and exits with code -1.
     let file = File::open("path/to/file")
         .map_mcp_crit("FSY-CRIT", format!("Can't open file '{}'", "path/to/file"))
         .or_exit();
     ```

3. **Benefits**

   - **Cleaner Code**: Reduce boilerplate by encapsulating error conversion into these methods.
   - **Consistency**: All parts of your project use a unified approach for error handling.
   - **Enhanced Debugging**: Automatically chains the source error, preserving the original error context.
   - **Immediate Failure for Non-recoverable Errors**: By chaining `.or_exit()`, the process terminates as soon as an error is encountered.

## Old Usage

### 1. Add as a dependency

In your `Cargo.toml`:

```toml
[dependencies]
mcp-error = { git = "ssh://git@github.com/mcp/mcp-error.git", tag = "v0.2.1" }
```

(Or reference your local path / desired branch.)

### 2. Creating a basic error

```rust
use mcp_error::{Error, Severity};

fn my_func() -> mcp_error::Result<()> {
    // Creating a basic error without a source
    let err = Error::new(Severity::Error, "NET-001", "Connection timed out");
    Err(err)
}
```

### 3. Adding a source error

If you have an `std::io::Error` or another error that implements `std::error::Error`, attach it to `Error`:

```rust
use mcp_error::{Error, Severity, Result};
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

## Why Use mcp-error?

- **Simplicity**: One flexible error type means fewer crates or enum variants to maintain.
- **Consistency**: All errors share the same fields for severity, reference codes, and messages.
- **Extensibility**: Crates can add domain-specific references, attach any error source, or store arbitrary metadata—without needing to change this crate.

## Contributing

Feel free to open pull requests or issues in the MCP Studio organization repository. The goal is to keep mcp-error small, so carefully consider if your feature truly needs to live here instead of your own crate.

## License

This crate is proprietary to the MCP Studio ecosystem (or add your preferred license statement here).
