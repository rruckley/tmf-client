//! TMF629 Module
//! Manage objects with TMF629 Customer Management API.

use tmflib::tmf629::customer::Customer;

use crate::{Operations,HasNew,Config};
use crate::common::tmf_error::TMFError;
use super::{
    create_tmf, get_tmf, list_tmf, update_tmf, delete_tmf
};

/// TMF629 Customer API
pub struct TMF629Customer {
    config : Config,
}

impl TMF629Customer {
    /// Create a new instance of the Customer module of TMF629 API
    pub fn new(config : Config) -> TMF629Customer {
        TMF629Customer { config }
    }
}

impl Operations for TMF629Customer {
    type TMF = Customer;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(&self.config,item)    
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf(&self.config,id.into())
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(&self.config,id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(&self.config,filter)  
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(&self.config,id.into(), patch)
    }
}

/// TMF629 Customer Management API
#[derive(Clone,Debug)]
pub struct TMF629 {
    config : Config,
}

impl HasNew<TMF629> for TMF629 {
    fn new(config : Config) -> TMF629 {
        TMF629 {
            config
        }  
    }
}

impl TMF629 {
    /// Access the Customer API
    pub fn customer(&mut self) -> TMF629Customer {
        TMF629Customer::new(self.config.clone())
    }
}