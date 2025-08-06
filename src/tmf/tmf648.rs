//! TMF648 Module

use tmflib::tmf648::quote::Quote;

use super::{create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf};
use crate::common::tmf_error::TMFError;
use crate::{Config, HasNew, Operations};

/// Interact with the quote object from TMF648
pub struct TMF648Quote {
    config: Config,
}

impl Operations for TMF648Quote {
    type TMF = Quote;

    fn create(&self, item: Self::TMF) -> Result<Self::TMF, TMFError> {
        create_tmf(&self.config, item)
    }
    fn delete(&self, id: impl Into<String>) -> Result<Self::TMF, TMFError> {
        delete_tmf(&self.config, id.into())
    }
    fn get(&self, id: impl Into<String>) -> Result<Vec<Self::TMF>, TMFError> {
        get_tmf(&self.config, id.into())
    }
    fn list(&self, filter: Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>, TMFError> {
        list_tmf(&self.config, filter)
    }
    fn update(&self, id: impl Into<String>, patch: Self::TMF) -> Result<Self::TMF, TMFError> {
        update_tmf(&self.config, id.into(), patch)
    }
}

/// Product Catalogue API
#[derive(Clone, Default, Debug)]
pub struct TMF648 {
    config: Config,
}

impl HasNew<TMF648> for TMF648 {
    fn new(config: Config) -> TMF648 {
        TMF648 { config }
    }
}

impl TMF648 {
    /// provide a quote API object to interact with
    pub fn quote(&self) -> TMF648Quote {
        TMF648Quote {
            config: self.config.clone(),
        }
    }
}
