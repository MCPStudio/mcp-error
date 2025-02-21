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

## Mapping Errors with Ephais Methods

Rather than mapping errors manually with `map_err`, the crate provides an extension trait with three methods to cover different use cases:

- **`.map_ephais_inf`**: Converts the error into an `Error` with Severity set to `Info` and returns a `Result<T>`.
- **`.map_ephais_err`**: For non-recoverable errors. On failure, it prints the error (with Severity set to `Error`) and exits the process with exit code `-1`.
- **`.map_ephais_crit`**: Similar to the previous method but uses a `Critical` severity.

### Why

Mapping errors manually with `map_err` can lead to repetitive code and potential inconsistencies throughout your projects. By providing these extension methods, you standardize error conversion across your ecosystem:
- **Cleaner Code**: Encapsulate error conversion in a single method.
- **Consistency**: Uniform error messages across your projects.
- **Immediate Failure**: For non-recoverable errors, the process immediately exits after printing a clear, formatted error message.

### What

The `EphErrorExt` trait adds these three methods to `Result<T, E>`:
- `.map_ephais_inf(reference, description) -> Result<T>`
- `.map_ephais_err(reference, description) -> T`
- `.map_ephais_crit(reference, description) -> T`

Each method converts any error implementing `std::error::Error` into your defined `Error` type. The conversion attaches:
- A severity level (set automatically according to the method)
- An error reference code (e.g., "FSY-404")
- A detailed human-readable description that includes the original error message
- The underlying error as the source

For `.map_ephais_err` and `.map_ephais_crit`, if an error occurs the error is printed via `eprintln!` and the program exits with code `-1`.

### How

1. **Import the Trait**

   In your consumer crates, import the trait along with the required types:

   ```rust
   use ephais_error::{Result};
   use ephais_error::EphErrorExt;
   ```

2. **Convert Errors Using the Methods**

   - **For Information-level errors (recoverable)**

     ```rust
     use std::fs::File;
     
     // Continues execution by propagating a converted error
     let file = File::open("path/to/file")
         .map_ephais_inf("FSY-INFO", format!("Can't open file '{}'", "path/to/file"))?;
     ```
   
   - **For Error-level failures (exit on error)**

     ```rust
     use std::fs::File;
     
     // On failure, prints the error with Severity::Error and exits with code -1.
     let file = File::open("path/to/file")
         .map_ephais_err("FSY-ERR", format!("Can't open file '{}'", "path/to/file"));
     ```

   - **For Critical-level failures (exit on error)**

     ```rust
     use std::fs::File;
     
     // On failure, prints the error with Severity::Critical and exits with code -1.
     let file = File::open("path/to/file")
         .map_ephais_crit("FSY-CRIT", format!("Can't open file '{}'", "path/to/file"));
     ```

3. **Benefits**

   - **Cleaner Code**: Reduce boilerplate by encapsulating error conversion into these methods.
   - **Consistency**: All parts of your project use a unified approach for error handling.
   - **Enhanced Debugging**: Automatically chains the source error, preserving the original error context.
   - **Immediate Failure for Non-recoverable Errors**: With `.map_ephais_err` and `.map_ephais_crit`, the process terminates as soon as an error is encountered.

## Old Usage

### 1. Add as a dependency

In your `Cargo.toml`:

```toml
[dependencies]
ephais-error = { git = "ssh://git@github.com/ephais/ephais-error.git", tag = "v0.2.1" }
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

