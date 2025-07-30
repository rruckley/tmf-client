//! TMF632 Party
//! 

use tmflib::tmf632::organization_v4::Organization;
use tmflib::Uri;
#[cfg(feature = "v4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "v5")]
use tmflib::tmf632::individual_v5::Individual;
use crate::{Operations,HasNew};
use crate::common::tmf_error::TMFError;
use super::{
    create_tmf, get_tmf, list_tmf, update_tmf, delete_tmf
};

/// TMF622 Product Order Object
pub struct TMF632Individual {
    /// End-point for TMF622
    host : Uri,
}

/// TMF632 Organization
pub struct TMF632Organization {
    host : Uri,
}

impl TMF632Individual {
    /// Create a new instance of Product Order API Object
    pub fn new(host : Uri) -> TMF632Individual {
        TMF632Individual{ host }
    }
}

impl Operations for TMF632Individual {
    type TMF = Individual;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(self.host.clone(), item)
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

impl TMF632Organization {
    /// Create a new instance of Product Order API Object
    pub fn new(host : Uri) -> TMF632Organization {
        TMF632Organization{ host }
    }
}

impl Operations for TMF632Organization {
    type TMF = Organization;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(self.host.clone(), item)    
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

/// Product Ordering API
#[derive(Clone,Default,Debug)]
pub struct TMF632 {
    host : Uri,
}

impl HasNew<TMF632> for TMF632 {
    fn new(host : Uri) -> TMF632 {
        TMF632 {
            host
        }       
    }
}

impl TMF632 {
    /// Access the order module of TMF622.
    pub fn individual(&self) -> TMF632Individual {
        TMF632Individual::new(self.host.clone())
    }

    /// Create a new Organisation API access object
    pub fn organization(&self) -> TMF632Organization {
        TMF632Organization::new(self.host.clone())
    }
}