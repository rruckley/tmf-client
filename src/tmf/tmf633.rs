//! TMF633 Module

use tmflib::Uri;
use tmflib::tmf633::service_candidate::ServiceCandidate;
use tmflib::tmf633::service_specification::ServiceSpecification;

use crate::common::tmf_error::TMFError;
use crate::{Operations,HasNew};
use super::{
    create_tmf,
    get_tmf,
    list_tmf,
};

/// ServiceCandidate API Object
pub struct TMF633Candidate {
    host: Uri,
}

impl Operations for TMF633Candidate {
    type TMF = ServiceCandidate;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(self.host.clone(),item)    
    }
    fn delete(&self, _id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented")) 
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)  
    }
    fn update(&self, _id : impl Into<String>, _patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented")) 
    }    
}

/// ServiceCatalog API Object
pub struct TMF633Catalog {
    host: Uri,
}

/// ServiceCategory API Object
pub struct TMF633Category {
    host: Uri,
}

/// ServiceSpecification API Object
pub struct TMF633Specification {
    host: Uri,
}

impl Operations for TMF633Specification {
    type TMF = ServiceSpecification;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(self.host.clone(),item)    
    }
    fn delete(&self, _id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)  
    }
    fn update(&self, _id : impl Into<String>, _patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented")) 
    }        
}

/// Product Catalogue API
#[derive(Clone,Default,Debug)]
pub struct TMF633 {
    host : Uri,
}

impl HasNew<TMF633> for TMF633 {
    fn new(host : Uri) -> TMF633 {
        TMF633 {
            host
        }
    }
}

impl TMF633 {
    /// Create ServiceCandidate API object
    pub fn candidate(&self) -> TMF633Candidate {
        TMF633Candidate { host: self.host.clone() }
    }

    /// Create ServiceCatalog API object
    pub fn catalog(&self) -> TMF633Catalog {
        TMF633Catalog { host: self.host.clone()}
    }

    /// Create ServiceCategory API Object
    pub fn category(&self) -> TMF633Category {
        TMF633Category { host: self.host.clone() }
    }

    /// Create ServiceSpecification API Object
    pub fn specification(&self) -> TMF633Specification {
        TMF633Specification { host: self.host.clone() }
    }
}