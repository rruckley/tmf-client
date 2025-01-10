//! TMF674 Module

use tmflib::Uri;
use tmflib::tmf674::geographic_site_v4::GeographicSite;

use crate::common::tmf_error::TMFError;
use crate::{Operations,HasNew};
use super::{
    create_tmf,
    get_tmf,
    list_tmf,
};

/// TMF674 GeographicSite API Object
pub struct TMF674GeographicSite {
    host : Uri,
}

impl Operations for TMF674GeographicSite {
    type TMF = GeographicSite;
    
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
pub struct TMF674 {
    host : Uri,
}

impl HasNew<TMF674> for TMF674 {
    fn new(host : Uri) -> TMF674 {
        TMF674 {
            host
        }
    }
}

impl TMF674 {
    /// Provide a GeographicSite API object
    pub fn site(&self) -> TMF674GeographicSite {
        TMF674GeographicSite {
            host: self.host.clone(),
        }
    }
}