//! TMF648 Module

use tmflib::Uri;
use tmflib::tmf648::quote::Quote;

use crate::common::tmf_error::TMFError;
use crate::{Operations,HasNew};
use super::{
    create_tmf,
    get_tmf,
    list_tmf,
};


/// Interact with the quote object from TMF648
pub struct TMF648Quote {
    host: Uri,
}

impl Operations for TMF648Quote {
    type TMF = Quote;

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
pub struct TMF648 {
    host : Uri,
}

impl HasNew<TMF648> for TMF648 {
    fn new(host : Uri) -> TMF648 {
        TMF648 {
            host
        }
    }

}

impl TMF648 {
    /// provide a quote API object to interact with
    pub fn quote(&self) -> TMF648Quote {
        TMF648Quote {
            host: self.host.clone(),
        }
    }
}