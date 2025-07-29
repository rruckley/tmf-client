
//! Error Module

use thiserror::Error;

/// TMF Error
#[derive(Debug,Error)]
pub enum TMFError {
    /// NoConnection Error when making a request to the TMF API
    #[error("TMFError: {0}")]
    NoConnection(String),
    /// Unknown Error when making a request to the TMF API
    #[error("TMFError: {0}")]
    Unknown(String),
    /// Serialization Error when serializing or deserializing a TMF object
    #[error("TMFError: {0}")]
    Serialization(String)
}

impl From<&str> for TMFError {
    fn from(value: &str) -> Self {
        TMFError::Unknown(value.to_string())
    }    
}

impl From<reqwest::Error> for TMFError {
    fn from(value: reqwest::Error) -> Self {
        TMFError::NoConnection(value.to_string())
    }
}

impl From<TMFError> for String {
    fn from(value: TMFError) -> Self {
        value.to_string()
    }
}

impl From<serde_json::Error> for TMFError {
    fn from(value: serde_json::Error) -> Self {
        TMFError::Serialization(value.to_string())
    }
}

impl From<std::io::Error> for TMFError {
    fn from(value: std::io::Error) -> Self {
        TMFError::NoConnection(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_from_str() {
        let err = TMFError::from("ThisIsAnError");

        assert_eq!(err.to_string(),"TMFError: ThisIsAnError".to_string());
    }

    #[test]
    fn test_string_from_error() {
        let err= TMFError::from("ThisIsAnError");

        let s : String = err.into();
        assert_eq!(s,"TMFError: ThisIsAnError".to_string());
    }
}