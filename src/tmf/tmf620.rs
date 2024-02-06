//! TMF620 Product Catalogue

use tmflib::tmf620::catalog::Catalog;
use tmflib::{HasId, Uri};
use crate::common::tmf_error::TMFError;

use crate::QueryOptions;

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
        let url = format!("{}{}",self.host,Catalog::get_class_href());
        let catalogs = reqwest::blocking::get(url)?.text()?;
        let output : Vec<Catalog> = serde_json::from_str(catalogs.as_str()).unwrap();
        Ok(output)
    }
    /// Get a list of catalogs applying optional filter
    pub fn list(&self, _filter : Option<QueryOptions>) -> Result<Vec<Catalog>,TMFError> {
        let url = format!("{}{}",self.host,Catalog::get_class_href());
        let catalogs = reqwest::blocking::get(url)?.text()?;
        let output : Vec<Catalog> = serde_json::from_str(catalogs.as_str()).unwrap();
        Ok(output)
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
}