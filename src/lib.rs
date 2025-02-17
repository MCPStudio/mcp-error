//! # ephais-error
//!
//! Central error handling crate for Ephais ecosystem projects and libraries.

use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

/// Error severity classification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Critical,
    Error,
    Warning,
    Info,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRIT"),
            Severity::Error => write!(f, "ERR"),
            Severity::Warning => write!(f, "WARN"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

/// Detailed error context.
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// A reference or code identifier for the error.
    pub reference: String,
    /// The severity of the error.
    pub severity: Severity,
    /// A human-readable error description.
    pub description: String,
    /// Additional metadata (optional).
    pub metadata: HashMap<String, String>,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | Ref: {} | {}",
            self.severity, self.reference, self.description
        )
    }
}

/// Core error type for the ecosystem.
#[derive(Debug)]
pub enum Error {
    Network(ErrorContext),
    DataFormat(ErrorContext),
    Unknown(ErrorContext),
    FileSystem(ErrorContext, Box<dyn StdError + Send + Sync>),
    External(ErrorContext, Box<dyn StdError + Send + Sync>),
}

/// Implement Display manually, rather than using `thiserror`.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(ctx) => write!(f, "[NETWORK] {}", ctx),
            Error::DataFormat(ctx) => write!(f, "[DATA FORMAT] {}", ctx),
            Error::Unknown(ctx) => write!(f, "[UNKNOWN] {}", ctx),
            Error::FileSystem(ctx, source) => {
                write!(f, "[FILE SYSTEM] {} | Source: {}", ctx, source)
            }
            Error::External(ctx, source) => {
                write!(f, "[EXTERNAL] {} | Source: {}", ctx, source)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        // If you have variants that wrap other errors, return Some(...) here
        match self {
            Error::FileSystem(_, src) => Some(&**src),
            Error::External(_, src) => Some(&**src),
            _ => None,
        }
    }
}

impl ErrorContext {
    fn new(reference: &str, severity: Severity, description: impl Into<String>) -> Self {
        Self {
            reference: reference.to_string(),
            severity,
            description: description.into(),
            metadata: HashMap::new(),
        }
    }
}

impl Error {
    /// Creates a new Network error.
    pub fn network(reference: &str, description: impl Into<String>) -> Self {
        Self::Network(ErrorContext::new(
            &format!("NET-{}", reference),
            Severity::Error,
            description,
        ))
    }

    /// Creates a new Data Format error.
    pub fn data_format(reference: &str, description: impl Into<String>) -> Self {
        Self::DataFormat(ErrorContext::new(
            &format!("FMT-{}", reference),
            Severity::Error,
            description,
        ))
    }

    /// Creates a File System error
    pub fn file_system(
        reference: &str,
        description: impl Into<String>,
        source: Box<dyn StdError + Send + Sync>,
    ) -> Self {
        Self::FileSystem(
            ErrorContext {
                reference: format!("FSY-{}", reference),
                severity: Severity::Error,
                description: description.into(),
                metadata: HashMap::new(),
            },
            source,
        )
    }

    /// Creates a new External error.
    pub fn external(
        reference: &str,
        description: impl Into<String>,
        source: Box<dyn StdError + Send + Sync>,
    ) -> Self {
        Self::External(
            ErrorContext {
                reference: format!("EXT-{}", reference),
                severity: Severity::Error,
                description: description.into(),
                metadata: HashMap::new(),
            },
            source,
        )
    }
}

/// A standard result type for the ecosystem.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn network_error_display() {
        let err = Error::network("TIMEOUT", "Connection timeout");
        assert_eq!(
            format!("{}", err),
            "[NETWORK] ERR | Ref: NET-TIMEOUT | Connection timeout"
        );
    }

    #[test]
    fn external_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let err = Error::external("FS-404", "Storage operation failed", Box::new(io_err));
        let output = format!("{}", err);
        println!("{:?}", output);
        assert!(output.contains(
            "[EXTERNAL] ERR | Ref: EXT-FS-404 | Storage operation failed | Source: File not found"
        ));
    }
}
