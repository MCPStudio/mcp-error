//! # ephais-error
//!
//! Central error handling crate for Ephais ecosystem projects and libraries.

use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Error severity classification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Critical,
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
#[derive(Debug, Error)]
pub enum Error {
    #[error("[NETWORK] {0}")]
    Network(ErrorContext),

    #[error("[DATA FORMAT] {0}")]
    DataFormat(ErrorContext),

    #[error("[UNKNOWN] {0}")]
    Unknown(ErrorContext),

    #[error("[EXTERNAL] {0} | Source: {1}")]
    External(
        ErrorContext,
        #[source] Box<dyn std::error::Error + Send + Sync>,
    ),
}

impl Error {
    /// Creates a new Network error.
    pub fn network(reference: &str, description: impl Into<String>) -> Self {
        Self::Network(ErrorContext {
            reference: format!("NET-{}", reference),
            severity: Severity::Error,
            description: description.into(),
            metadata: HashMap::new(),
        })
    }

    /// Creates a new Data Format error.
    pub fn data_format(reference: &str, description: impl Into<String>) -> Self {
        Self::DataFormat(ErrorContext {
            reference: format!("FMT-{}", reference),
            severity: Severity::Error,
            description: description.into(),
            metadata: HashMap::new(),
        })
    }

    /// Creates a new External error.
    pub fn external(
        reference: &str,
        description: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
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
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let err = Error::external("FS-404", "Storage operation failed", Box::new(io_err));
        let output = format!("{}", err);
        println!("{:?}", output);
        assert!(output.contains(
            "[EXTERNAL] ERR | Ref: EXT-FS-404 | Storage operation failed | Source: File not found"
        ));
    }
}
