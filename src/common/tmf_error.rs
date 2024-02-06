
//! Error Module

#[derive(Debug)]
pub struct TMFError {
    message : String,
}

impl From<&str> for TMFError {
    fn from(value: &str) -> Self {
        TMFError {
            message : value.into(),
        }
    }    
}

impl From<reqwest::Error> for TMFError {
    fn from(value: reqwest::Error) -> Self {
        TMFError {
            message : value.to_string(),
        }
    }
}