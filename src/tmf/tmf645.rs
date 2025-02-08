//! TMF645 Service Qualifcation API
//! Manage objects with TMF645 Service Qualification API

use tmflib::tmf629::customer::Customer;
use tmflib::tmf645::check_service_qualification::CheckServiceQualification;
use tmflib::Uri;

use crate::{Operations,HasNew};
use crate::common::tmf_error::TMFError;
use super::{
    create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf
};

/// TMF645 Service Qualification API
pub struct TMF645CheckServiceQualification {
    host : Uri,
}

impl TMF645CheckServiceQualification {
    /// Create a new instance of the Service Qualificatoin module of TMF645 API
    pub fn new(host : Uri) -> TMF645CheckServiceQualification {
        TMF645CheckServiceQualification { host }
    }
}

impl Operations for TMF645CheckServiceQualification {
    type TMF = CheckServiceQualification;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(self.host.clone(),item)    
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf(self.host.clone(),id.into())
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)  
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(self.host.clone(),id.into(), patch)
    }
}

/// TMF645 Service Qualification API
#[derive(Clone,Default,Debug)]
pub struct TMF645 {
    host : Uri,
}

impl HasNew<TMF645> for TMF645 {
    fn new(host : Uri) -> TMF645 {
        TMF645 {
            host
        }
    }
}

impl TMF645 {
    /// Access the Check Service Qualification API
    pub fn check_qualifcation(&mut self) -> TMF645CheckServiceQualification {
        TMF645CheckServiceQualification::new(self.host.clone())
    }
}
