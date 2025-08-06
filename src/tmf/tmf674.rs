//! TMF674 Module

use tmflib::tmf674::geographic_site_v4::GeographicSite;

use super::{create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf};
use crate::common::tmf_error::TMFError;
use crate::{Config, HasNew, Operations};

/// TMF674 GeographicSite API Object
pub struct TMF674GeographicSite {
    config: Config,
}

impl Operations for TMF674GeographicSite {
    type TMF = GeographicSite;

    fn create(&self, item: Self::TMF) -> Result<Self::TMF, TMFError> {
        create_tmf(&self.config, item)
    }
    fn delete(&self, id: impl Into<String>) -> Result<Self::TMF, TMFError> {
        delete_tmf(&self.config, id)
    }
    fn get(&self, id: impl Into<String>) -> Result<Vec<Self::TMF>, TMFError> {
        get_tmf(&self.config, id.into())
    }
    fn list(&self, filter: Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>, TMFError> {
        list_tmf(&self.config, filter)
    }
    fn update(&self, id: impl Into<String>, patch: Self::TMF) -> Result<Self::TMF, TMFError> {
        update_tmf(&self.config, id, patch)
    }
}

/// Product Catalogue API
#[derive(Clone, Default, Debug)]
pub struct TMF674 {
    config: Config,
}

impl HasNew<TMF674> for TMF674 {
    fn new(config: Config) -> TMF674 {
        TMF674 { config }
    }
}

impl TMF674 {
    /// Provide a GeographicSite API object
    pub fn site(&self) -> TMF674GeographicSite {
        TMF674GeographicSite {
            config: self.config.clone(),
        }
    }
}
