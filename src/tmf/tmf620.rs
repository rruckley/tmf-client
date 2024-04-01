//! TMF620 Product Catalogue


use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::Uri;
use super::{get_tmf,list_tmf,create_tmf,update_tmf,delete_tmf};
use crate::common::tmf_error::TMFError;

use crate::{QueryOptions,Operations};

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
}

impl Operations for TMF620Category {
    type TMF = Category;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(self.host.clone(), item)   
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf(self.host.clone(), id)        
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())    
    }
    fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)    
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(self.host.clone(), id, patch)
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
}

impl Operations for TMF620Catalog {
    type TMF = Catalog;

    fn create(&self, _item : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))    
    }
    fn delete(&self, _id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))    
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())    
    }
    fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)    
    }
    fn update(&self, _id : impl Into<String>, _patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))    
    }
}



/// TMF620 ProductOffering API calls
#[derive(Clone,Default,Debug)]
pub struct TMF620ProductOffering {
    host : Uri,
}

impl TMF620ProductOffering {
    /// Create a new product_offering reference
    pub fn new(host : Uri) -> TMF620ProductOffering {
        TMF620ProductOffering { host }
    }
}

impl Operations for TMF620ProductOffering {
    type TMF = ProductOffering;

    fn create(&self, _item : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))    
    }
    fn delete(&self, _id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))        
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())    
    }
    fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)    
    }
    fn update(&self, _id : impl Into<String>, _patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))
    }
}

/// TMF620 ProductOffering API calls
#[derive(Clone,Default,Debug)]
pub struct TMF620ProductOfferingPrice {
    host : Uri,
}

impl TMF620ProductOfferingPrice {
    /// Create a new product_offering reference
    pub fn new(host : Uri) -> TMF620ProductOfferingPrice {
        TMF620ProductOfferingPrice { host }
    }
}

impl Operations for TMF620ProductOfferingPrice {
    type TMF = ProductOfferingPrice;

    fn create(&self, _item : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))    
    }
    fn delete(&self, _id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))        
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())    
    }
    fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)    
    }
    fn update(&self, _id : impl Into<String>, _patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))
    }
}

/// TMF620 ProductSpecification API calls
#[derive(Clone,Default,Debug)]
pub struct TMF620ProductSpecification {
    host : Uri,
}

impl TMF620ProductSpecification {
    /// Create a new product_offering reference
    pub fn new(host : Uri) -> TMF620ProductSpecification {
        TMF620ProductSpecification { host }
    }
}

impl Operations for TMF620ProductSpecification {
    type TMF = ProductSpecification;

    fn create(&self, _item : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))    
    }
    fn delete(&self, _id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))        
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())    
    }
    fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)    
    }
    fn update(&self, _id : impl Into<String>, _patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        Err(TMFError::from("Not implemented"))
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

    /// Return function for managing product_offering
    pub fn product_offering(&self) -> TMF620ProductOffering {
        TMF620ProductOffering::new(self.host.clone())
    }

    /// Return function for managing product_offering
    pub fn product_offering_price(&self) -> TMF620ProductOfferingPrice {
        TMF620ProductOfferingPrice::new(self.host.clone())
    }

    /// Return function for managing product_offering
    pub fn product_specification(&self) -> TMF620ProductSpecification {
        TMF620ProductSpecification::new(self.host.clone())
    }
}