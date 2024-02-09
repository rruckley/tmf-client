//! TMF620 Product Catalogue

use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::category::Category;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::Uri;
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
    /// ```
    /// # use tmf_client::TMFClient;
    /// let categories = TMFClient::new("http://localhost:8000")
    ///     .tmf620()
    ///     .category()
    ///     .list(None);
    /// ```
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

    /// Get a single product offering
    pub fn get(&self, _id : impl Into<String>) -> Result<Vec<ProductOffering>,TMFError> {
        get_tmf(self.host.clone(),_id.into())
    }

    /// Get a list of catalogs applying optional filter
    pub fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<ProductOffering>,TMFError> {
        list_tmf(self.host.clone(),filter)
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

    /// Get a single product offering
    pub fn get(&self, _id : impl Into<String>) -> Result<Vec<ProductOfferingPrice>,TMFError> {
        get_tmf(self.host.clone(),_id.into())
    }

    /// Get a list of catalogs applying optional filter
    pub fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<ProductOfferingPrice>,TMFError> {
        list_tmf(self.host.clone(),filter)
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

    /// Get a single product offering
    pub fn get(&self, _id : impl Into<String>) -> Result<Vec<ProductSpecification>,TMFError> {
        get_tmf(self.host.clone(),_id.into())
    }

    /// Get a list of catalogs applying optional filter
    pub fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<ProductSpecification>,TMFError> {
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