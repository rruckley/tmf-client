
//! Error Module

#[derive(Debug,Default)]
pub struct TMFError {
    pub message : String,
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

impl From<TMFError> for String {
    fn from(value: TMFError) -> Self {
        format!("TMFError: {}",value.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_from_str() {
        let err = TMFError::from("ThisIsAnError");

        assert_eq!(err.message,"ThisIsAnError".to_string());
    }

    #[test]
    fn test_string_from_error() {
        let err = TMFError {
            message : "ThisIsAnError".into(),
        };

        let s : String = err.into();
        assert_eq!(s,"TMFError: ThisIsAnError".to_string());
    }
}