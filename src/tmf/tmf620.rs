//! TMF620 Product Catalogue

use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;
use tmflib::{HasId, Uri};
use super::{get_tmf,list_tmf};
use crate::common::tmf_error::TMFError;

use crate::QueryOptions;

/// TMF620 Category API calls
#[derive(Clone,Default,Debug)]
pub struct TMF620Category {
    host : Uri,
}

impl TMF620Category {
    /// Create a new category reference
    pub fn new(host : Uri) -> TMF620Category {
        TMF620Category { host }
    }
    /// Get a single catalog entry
    pub fn get(&self, _id : impl Into<String>) -> Result<Vec<Category>,TMFError> {
        get_tmf(self.host.clone(),_id.into())
    }
    /// Get a list of catalogs applying optional filter
    pub fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Category>,TMFError> {
        list_tmf(self.host.clone(),filter)
    }
}

/// TMF620 Catalog API calls
#[derive(Clone,Default,Debug)]
pub struct TMF620Catalog {
    host : Uri,
}



impl TMF620Catalog {
    /// Create a new catalog reference
    pub fn new(host : Uri) -> TMF620Catalog {
        TMF620Catalog { host }
    }
    /// Get a single catalog entry
    pub fn get(&self, _id : impl Into<String>) -> Result<Vec<Catalog>,TMFError> {
        get_tmf(self.host.clone(),_id.into())
    }
    /// Get a list of catalogs applying optional filter
    pub fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Catalog>,TMFError> {
        list_tmf(self.host.clone(),filter)
    }
}

/// Product Catalogue API
#[derive(Clone,Default,Debug)]
pub struct TMF620 {
    host : Uri,
}

impl TMF620 {
    /// Create a new instance of TMF620 API
    pub fn new(host : Uri) -> TMF620 {
        TMF620 {
            host
        }
    }
    /// Return function for managing catalogs
    pub fn catalog(&self) -> TMF620Catalog {
        TMF620Catalog::new(self.host.clone())
    }
    /// Return function for managing categories
    pub fn category(&self) -> TMF620Category {
        TMF620Category::new(self.host.clone())
    }
}