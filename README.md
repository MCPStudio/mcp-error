# ephais-error

Central error handling crate for Ephais ecosystem projects and libraries.

---
---

## Overview

`ephais-error` is a Rust crate designed to centralize error handling within the Ephais ecosystem. It standardizes how errors are created and displayed through a clear structure including:
- Error severity classification
- Error context containing a unique reference, readable description, and optional metadata field
- Five error variants:
    * `Network`
    * `DataFormat`
    * `FileSystem`
    * `Unknown`
    * `External`

---
---

## Features

- **Centralized error handling**: Through a common `ErrorContext`, each error contains detailed information (reference, severity, description, metadata)
- **Severity levels**: Distinguish error importance (Critical, Error, Warning, Info)
- **Specific error variants**: Each error type (network, format, filesystem, unknown and external) is defined as a variant of the `Error` enum
- **Type alias**: A `Result<T>` alias is provided to simplify result handling in your functions

---
---

## Installation

Add `ephais-error` to your project by modifying your `Cargo.toml`:

```toml
[dependencies]
ephais-error = "0.1.0" # Current version  
```

---
---

## Error Structure

### ErrorContext

Each error contains an ErrorContext with the following fields:

* reference: An identifier or code, e.g., "NET-TIMEOUT" or "EXT-FS-404"
* severity: Severity level (Critical, Error, Warning, Info)
* description: A detailed message describing the error
* metadata: A HashMap for additional information (optional)

### Error Variants

* Network: For network connection-related errors
* DataFormat: For data formatting or parsing errors
* FileSystem : For file system related error (files, folders, etc)
* Unknown: For unclassified errors
* External: For wrapping external errors (such as those from other systems or libraries)

> **Example:**
> 
> For instance, when wrapping an external error, you would explicitly call the `Error::external()` method:
> 
> ```rust
> let fs_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
> let err = Error::external("FS-404", "Storage operation failed", Box::new(fs_error));
> println!("{}", err);
> // Expected output:
> // "[EXTERNAL] ERR | Ref: EXT-FS-404 | Storage operation failed | Source: File not found"
> ```
>
> **Usage with a `Result`**:
> ```rust
> use ephais_error::{Error, Result};
> use reqwest::Client;
> 
> async fn fetch_user_data(user_id: &str) -> Result<String> {
>     if user_id.is_empty() {
>         return Err(Error::network(
>             "NET-INVALID",
>             "User ID cannot be empty",
>             None
>         ));
>     }
>     
>     let client = Client::new();
>     match client
>         .get(&format!("https://api.example.com/users/{}", user_id))
>         .timeout(std::time::Duration::from_secs(5))
>         .send()
>         .await
>     {
>         Ok(response) => Ok(response.text().await.unwrap_or_default()),
>         Err(e) => Err(Error::network(
>             "NET-TIMEOUT",
>             "Failed to fetch user data",
>             Some(std::collections::HashMap::from([
>                 ("user_id".to_string(), user_id.to_string()),
>                 ("error".to_string(), e.to_string())
>             ]))
>         ))
>     }
> }
> 
> #[tokio::main]
> async fn main() {
>     match risky_operation() {
>         Ok(value) => println!("Operation succeeded: {}", value),
>         Err(err) => eprintln!("Operation failed: {}", err),
>     }
> }
> ```

---
---

## Tests

The crate includes unit tests to validate error formatting and construction. For example:

It tests the creation of a network error and of an external error.

---
---

## Licence

For the exclusive use of Ephais SAS and with reserved rights.