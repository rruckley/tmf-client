//! TMF637 Product Inventory Management API

use tmflib::tmf637::v4::product::Product;

use crate::{Operations,HasNew,Config};
use crate::common::tmf_error::TMFError;
use super::{
    create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf
};

// /// TMF645 Service Qualification API
// #[derive(Clone,Default,Debug)]
// pub struct TMF645 {
//     host : Uri,
// }

// impl HasNew<TMF645> for TMF645 {
//     fn new(host : Uri) -> TMF645 {
//         TMF645 {
//             host
//         }
//     }
// }

// impl TMF645 {
//     /// Access the Check Service Qualification API
//     pub fn check_qualifcation(&mut self) -> TMF645CheckServiceQualification {
//         TMF645CheckServiceQualification::new(self.config)
//     }
// }

/// TMF637 Product Inventory Management API
pub struct TMF637ProductInventoryManagement {
    config: Config,
}

impl TMF637ProductInventoryManagement {
    /// Create a new instance of the Product Inventory Management module of TMF637 API
    pub fn new(config: Config) -> TMF637ProductInventoryManagement {
        TMF637ProductInventoryManagement { config }
    }
}

impl Operations for TMF637ProductInventoryManagement {
    type TMF = Product;

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

/// TMF637 Product Inventory Management API
#[derive(Clone,Default,Debug)]
pub struct TMF637 {
    config: Config,
}

impl HasNew<TMF637> for TMF637 {
    fn new(config: Config) -> TMF637 {
        TMF637 { config }
    }
}

impl TMF637 {
    /// Access the Product Inventory Management API
    pub fn product(&mut self) -> TMF637ProductInventoryManagement {
        super::tmf637::TMF637ProductInventoryManagement::new(self.config.clone())
    }
}