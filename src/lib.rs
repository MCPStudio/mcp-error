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
    /// Network-related errors (optionally wrapping a source error)
    Network(ErrorContext, Option<Box<dyn StdError + Send + Sync>>),
    /// Data format errors (optionally wrapping a source error)
    DataFormat(ErrorContext, Option<Box<dyn StdError + Send + Sync>>),
    /// Unknown errors (optionally wrapping a source error)
    Unknown(ErrorContext, Option<Box<dyn StdError + Send + Sync>>),
    /// File-system-related errors, always wrapping a source error
    FileSystem(ErrorContext, Box<dyn StdError + Send + Sync>),
    /// External errors, always wrapping a source error
    External(ErrorContext, Box<dyn StdError + Send + Sync>),
    /// SCP-related errors (optionally wrapping a source error)
    Scp(ErrorContext, Option<Box<dyn StdError + Send + Sync>>),
}

/// Implement Display manually, rather than using `thiserror`.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(ctx, Some(src)) => {
                write!(f, "[NETWORK] {} | Source: {}", ctx, src)
            }
            Error::Network(ctx, None) => {
                write!(f, "[NETWORK] {}", ctx)
            }
            Error::DataFormat(ctx, Some(src)) => {
                write!(f, "[DATA FORMAT] {} | Source: {}", ctx, src)
            }
            Error::DataFormat(ctx, None) => {
                write!(f, "[DATA FORMAT] {}", ctx)
            }
            Error::Unknown(ctx, Some(src)) => {
                write!(f, "[UNKNOWN] {} | Source: {}", ctx, src)
            }
            Error::Unknown(ctx, None) => {
                write!(f, "[UNKNOWN] {}", ctx)
            }
            Error::FileSystem(ctx, src) => {
                write!(f, "[FILE SYSTEM] {} | Source: {}", ctx, src)
            }
            Error::External(ctx, src) => {
                write!(f, "[EXTERNAL] {} | Source: {}", ctx, src)
            }
            Error::Scp(ctx, Some(src)) => {
                write!(f, "[SCP] {} | Source: {}", ctx, src)
            }
            Error::Scp(ctx, None) => {
                write!(f, "[SCP] {}", ctx)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            // If we have an optional Box, return Some if present
            Error::Network(_, Some(src))
            | Error::DataFormat(_, Some(src))
            | Error::Unknown(_, Some(src))
            | Error::Scp(_, Some(src)) => Some(&**src),

            // Always a source here
            Error::FileSystem(_, src) | Error::External(_, src) => Some(&**src),

            // No source was set
            Error::Network(_, None)
            | Error::DataFormat(_, None)
            | Error::Unknown(_, None)
            | Error::Scp(_, None) => None,
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
    /// Creates a new Network error (no source)
    pub fn network(reference: &str, description: impl Into<String>) -> Self {
        Self::Network(
            ErrorContext::new(&format!("NET-{}", reference), Severity::Error, description),
            None,
        )
    }

    /// Creates a new Network error (with a source)
    pub fn network_with_source(
        reference: &str,
        description: impl Into<String>,
        source: Box<dyn StdError + Send + Sync>,
    ) -> Self {
        Self::Network(
            ErrorContext::new(&format!("NET-{}", reference), Severity::Error, description),
            Some(source),
        )
    }

    /// Creates a new Data Format error (no source)
    pub fn data_format(reference: &str, description: impl Into<String>) -> Self {
        Self::DataFormat(
            ErrorContext::new(&format!("FMT-{}", reference), Severity::Error, description),
            None,
        )
    }

    /// Creates a new Data Format error (with a source)
    pub fn data_format_with_source(
        reference: &str,
        description: impl Into<String>,
        source: Box<dyn StdError + Send + Sync>,
    ) -> Self {
        Self::DataFormat(
            ErrorContext::new(&format!("FMT-{}", reference), Severity::Error, description),
            Some(source),
        )
    }

    /// Creates a new Unknown error (no source)
    pub fn unknown(reference: &str, description: impl Into<String>, severity: Severity) -> Self {
        Self::Unknown(
            ErrorContext::new(&format!("UNK-{}", reference), severity, description),
            None,
        )
    }

    /// Creates a new Unknown error (with a source)
    pub fn unknown_with_source(
        reference: &str,
        description: impl Into<String>,
        severity: Severity,
        source: Box<dyn StdError + Send + Sync>,
    ) -> Self {
        Self::Unknown(
            ErrorContext::new(&format!("UNK-{}", reference), severity, description),
            Some(source),
        )
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

    /// Creates a new SCP error (no source)
    pub fn scp(reference: &str, description: impl Into<String>) -> Self {
        Self::Scp(
            ErrorContext::new(&format!("SCP-{}", reference), Severity::Error, description),
            None,
        )
    }

    /// Creates a new SCP error (with a source)
    pub fn scp_with_source(
        reference: &str,
        description: impl Into<String>,
        source: Box<dyn StdError + Send + Sync>,
    ) -> Self {
        Self::Scp(
            ErrorContext::new(&format!("SCP-{}", reference), Severity::Error, description),
            Some(source),
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
    fn network_error_no_source_display() {
        let err = Error::network("TIMEOUT", "Connection timeout");
        assert_eq!(
            format!("{}", err),
            "[NETWORK] ERR | Ref: NET-TIMEOUT | Connection timeout"
        );
    }

    #[test]
    fn network_error_with_source_display() {
        let io_err = io::Error::new(io::ErrorKind::TimedOut, "Timed out");
        let err = Error::network_with_source("TIMEOUT", "Connection timed out", Box::new(io_err));
        let out = format!("{}", err);
        assert!(out.contains(
            "[NETWORK] ERR | Ref: NET-TIMEOUT | Connection timed out | Source: Timed out"
        ));
    }

    #[test]
    fn file_system_error_display() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
        let fs_err = Error::file_system("PERM", "Cannot access file", Box::new(io_err));
        let out = format!("{}", fs_err);
        assert!(out.contains(
            "[FILE SYSTEM] ERR | Ref: FSY-PERM | Cannot access file | Source: Permission denied"
        ));
    }

    #[test]
    fn unknown_error_no_source_display() {
        let err = Error::unknown("X123", "Unexpected issue", Severity::Warning);
        assert_eq!(
            format!("{}", err),
            "[UNKNOWN] WARN | Ref: UNK-X123 | Unexpected issue"
        );
    }

    #[test]
    fn unknown_error_with_source_display() {
        let parse_err = "Bad parse".to_string();
        // Using an std::io::Error to wrap the string, which does implement std::error::Error
        let wrapped_err = std::io::Error::new(std::io::ErrorKind::Other, parse_err.clone());
        let err = Error::unknown_with_source(
            "X999",
            "Strange parse error",
            Severity::Warning,
            Box::new(wrapped_err),
        );
        let out = format!("{}", err);
        assert!(out
            .contains("[UNKNOWN] WARN | Ref: UNK-X999 | Strange parse error | Source: Bad parse"));
    }
}
