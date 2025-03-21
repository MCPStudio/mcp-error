//! # mcp-error
//!
//! A minimal shared error-handling crate for the MCP Studio ecosystem.
//!
//! ## Overview
//! - A single `Error` struct with optional source error
//! - A `Severity` enum for classification
//! - `Result<T> = std::result::Result<T, Error>`
//! - Crates can attach specific references (like \"NET-001\", \"FSY-404\"), set severity, add metadata, etc.

use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

use serde::{
    ser::{SerializeStruct},
    Serializer,
};
use serde_derive::Serialize;

/// Indicates how severe an error is.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
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

/// A minimal, flexible error type for the Ephais ecosystem.
#[derive(Debug, Serialize)]
pub struct Error {
    /// Severity of the error (Error, Warning, Info, etc.).
    pub severity: Severity,
    /// Short code or reference, e.g. \"NET-001\" or \"FSY-404\".
    pub reference: String,
    /// A human-readable error description.
    pub description: String,
    /// Optional metadata for additional context.
    pub metadata: HashMap<String, String>,
    /// Optional underlying source error.
    #[serde(serialize_with = "serialize_source")]
    source: Option<Box<dyn StdError + Send + Sync>>,
}

fn serialize_source<S>(
    source: &Option<Box<dyn StdError + Send + Sync>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(source) = source {
        let mut state = serializer.serialize_struct("Source", 1)?;
        state.serialize_field("message", &source.to_string())?;
        state.end()
    } else {
        serializer.serialize_none()
    }
}

impl Error {
    /// Creates a new `Error` without a source.
    pub fn new<S1, S2>(severity: Severity, reference: S1, description: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            severity,
            reference: reference.into(),
            description: description.into(),
            metadata: HashMap::new(),
            source: None,
        }
    }

    /// Adds or replaces the source error in an existing `Error`.
    pub fn with_source(mut self, source: Box<dyn StdError + Send + Sync>) -> Self {
        self.source = Some(source);
        self
    }

    /// Inserts a key/value pair into `metadata`.
    pub fn insert_metadata<M: Into<String>, N: Into<String>>(mut self, key: M, value: N) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns a reference to the underlying source error, if any.
    pub fn get_source(&self) -> Option<&(dyn StdError + Send + Sync)> {
        self.source.as_deref()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Example output:
        // [ERR] Ref: NET-001 | description
        // Optionally show source error: ... | Source: {source}
        match &self.source {
            Some(src) => {
                write!(
                    f,
                    "[{}] Ref: {} | {} | Source: {}",
                    self.severity, self.reference, self.description, src
                )
            }
            None => {
                write!(
                    f,
                    "[{}] Ref: {} | {}",
                    self.severity, self.reference, self.description
                )
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|e| e as &(dyn StdError + 'static))
    }
}

/// A convenient type alias for results that return `Error`.
pub type Result<T> = std::result::Result<T, Error>;

pub trait EphErrorExt<T> {
    /// For non-critical (Info) errors: converts the error into an `Error` with Severity::Info.
    fn map_mcp_inf(
        self,
        reference: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<T>;

    /// For Error-level failures: propagate the error.
    fn map_mcp_err(
        self,
        reference: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<T>;

    /// For Critical-level failures: propagate the error.
    fn map_mcp_crit(
        self,
        reference: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<T>;
}

impl<T, E> EphErrorExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn map_mcp_inf(
        self,
        reference: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<T> {
        self.map_err(|e| {
            Error::new(
                Severity::Info,
                reference,
                format!("{}: {}", description.into(), e),
            )
            .with_source(Box::new(e))
        })
    }

    fn map_mcp_err(
        self,
        reference: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<T> {
        self.map_err(|e| {
            Error::new(
                Severity::Error,
                reference,
                format!("{}: {}", description.into(), e),
            )
            .with_source(Box::new(e))
        })
    }

    fn map_mcp_crit(
        self,
        reference: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<T> {
        self.map_err(|e| {
            Error::new(
                Severity::Critical,
                reference,
                format!("{}: {}", description.into(), e),
            )
            .with_source(Box::new(e))
        })
    }
}

pub trait OrExit<T> {
    fn or_exit(self) -> T;
}

impl<T, E> OrExit<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn or_exit(self) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(-1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::io;

    #[test]
    fn create_basic_error() {
        let err = Error::new(Severity::Error, "NET-001", "Timeout");
        assert_eq!(err.severity, Severity::Error);
        assert_eq!(err.reference, "NET-001");
        assert_eq!(err.description, "Timeout");
        assert!(err.source.is_none());
    }

    #[test]
    fn attach_source_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let mcp_err = Error::new(Severity::Error, "FSY-404", "Cannot read file")
            .with_source(Box::new(io_err));

        // The Display should include the source:
        let out = format!("{}", mcp_err);
        assert!(out.contains("Source: File not found"));

        // The source() method should return Some
        assert!(mcp_err.source().is_some());
    }

    #[test]
    fn insert_metadata() {
        let mut err = Error::new(Severity::Warning, "DS-002", "Data parse incomplete");
        err = err.insert_metadata("filename", "data.json");
        err = err.insert_metadata("line", "42");

        assert_eq!(err.metadata["filename"], "data.json");
        assert_eq!(err.metadata["line"], "42");
    }

    #[test]
    fn serialize_error() {
        let err = Error::new(Severity::Error, "NET-001", "Timeout");
        let serialized = serde_json::to_string(&err).unwrap();
        let expected = r#"{"severity":"Error","reference":"NET-001","description":"Timeout","metadata":{},"source":null}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn serialize_error_with_source() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let ephais_err = Error::new(Severity::Error, "FSY-404", "Cannot read file")
            .with_source(Box::new(io_err));

        let serialized = serde_json::to_string(&ephais_err).unwrap();
        let expected = r#"{"severity":"Error","reference":"FSY-404","description":"Cannot read file","metadata":{},"source":{"message":"File not found"}}"#;
        assert_eq!(serialized, expected);
    }
}
