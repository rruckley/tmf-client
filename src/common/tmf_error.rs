
//! Error Module

use thiserror::Error;

#[derive(Debug,Error)]
pub enum TMFError {
    #[error("TMFError: {0}")]
    NoConnection(String),
    #[error("TMFError: {0}")]
    Unknown(String),
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