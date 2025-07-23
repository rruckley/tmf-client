//! TMF638 Service Inventory Management API

// use tmflib::tmf637::v4::product::Product;
use tmflib::tmf638::service::Service;
use tmflib::Uri;

use crate::{Operations,HasNew};
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
//         TMF645CheckServiceQualification::new(self.host.clone())
//     }
// }

/// TMF638 Service Inventory Management API
pub struct TMF638ServiceInventoryManagement {
    host: Uri,
}

impl TMF638ServiceInventoryManagement {
    /// Create a new instance of the Service Inventory Management module of TMF638 API
    pub fn new(host: Uri) -> TMF638ServiceInventoryManagement {
        TMF638ServiceInventoryManagement { host }
    }
}

impl Operations for TMF638ServiceInventoryManagement {
    type TMF = Service;

    fn create(&self, item: Self::TMF) -> Result<Self::TMF, TMFError> {
        create_tmf(self.host.clone(), item)
    }
    fn delete(&self, id: impl Into<String>) -> Result<Self::TMF, TMFError> {
        delete_tmf(self.host.clone(), id.into())
    }
    fn get(&self, id: impl Into<String>) -> Result<Vec<Self::TMF>, TMFError> {
        get_tmf(self.host.clone(), id.into())
    }
    fn list(&self, filter: Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>, TMFError> {
        list_tmf(self.host.clone(), filter)
    }
    fn update(&self, id: impl Into<String>, patch: Self::TMF) -> Result<Self::TMF, TMFError> {
        update_tmf(self.host.clone(), id.into(), patch)
    }
}

/// TMF638 API
/// This module provides access to the Service Inventory Management API of TMF638.
#[derive(Clone,Default,Debug)]
pub struct TMF638 {
    host: Uri,
}

impl HasNew<TMF638> for TMF638 {
    fn new(host: Uri) -> TMF638 {
        TMF638 { host }
    }
}

impl TMF638 {
    /// Access the Product Inventory Management API
    pub fn service(&mut self) -> TMF638ServiceInventoryManagement {
        super::tmf638::TMF638ServiceInventoryManagement::new(self.host.clone())
    }
}