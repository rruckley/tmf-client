//! TMF639 Resource Inventory Management API

// use tmflib::tmf637::v4::product::Product;
use tmflib::tmf639::resource::Resource;

use crate::{Operations,HasNew,Config};
use crate::common::tmf_error::TMFError;
use super::{
    create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf
};

/// TMF639 Resource Inventory Management API
pub struct TMF639ResourceInventoryManagement {
    config: Config,
}

impl TMF639ResourceInventoryManagement {
    /// Create a new instance of the Resource Inventory Management module of TMF639 API
    pub fn new(config: Config) -> TMF639ResourceInventoryManagement {
        TMF639ResourceInventoryManagement { config }
    }
}

impl Operations for TMF639ResourceInventoryManagement {
    type TMF = Resource;

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

/// TMF639 API
/// This module provides access to the Resource Inventory Management API of TMF639.
#[derive(Clone,Default,Debug)]
pub struct TMF639 {
    config: Config,
}

impl HasNew<TMF639> for TMF639 {
    fn new(config: Config) -> TMF639 {
        TMF639 { config }
    }
}

impl TMF639 {
    /// Access the Product Inventory Management API
    pub fn resource(&mut self) -> TMF639ResourceInventoryManagement {
        super::tmf639::TMF639ResourceInventoryManagement::new(self.config.clone())
    }
}