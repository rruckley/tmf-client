//! TMF629 Module
//! Manage objects with TMF629 Customer Management API.

use tmflib::tmf629::customer::Customer;
use tmflib::Uri;

use crate::{Operations,HasNew};
use crate::common::tmf_error::TMFError;
use super::{
    create_tmf, get_tmf, list_tmf
};

/// TMF629 Customer API
pub struct TMF629Customer {
    host  :Uri,
}

impl TMF629Customer {
    /// Create a new instance of the Customer module of TMF629 API
    pub fn new(host : Uri) -> TMF629Customer {
        TMF629Customer { host }
    }
}

impl Operations for TMF629Customer {
    type TMF = Customer;

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

/// TMF629 Customer Management API
#[derive(Clone,Default,Debug)]
pub struct TMF629 {
    host : Uri,
}

impl HasNew<TMF629> for TMF629 {
    fn new(host : Uri) -> TMF629 {
        TMF629 {
            host
        }  
    }
}

impl TMF629 {
    /// Access the Customer API
    pub fn customer(&mut self) -> TMF629Customer {
        TMF629Customer::new(self.host.clone())
    }
}