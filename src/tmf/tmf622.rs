//! TMF622 Product Order
use tmflib::{tmf622::product_order_v4::ProductOrder, Uri};
use crate::Operations;
use crate::common::tmf_error::TMFError;
use super::{
    get_tmf,
    list_tmf
};

/// TMF622 Product Order Object
pub struct TMF622ProductOrder {
    /// End-point for TMF622
    host : Uri,
}

impl Operations for TMF622ProductOrder {
    type TMF = ProductOrder;

    fn create(&self, _item : Self::TMF) -> Result<Self::TMF,crate::common::tmf_error::TMFError> {
        Err(TMFError::from("Not implemented"))    
    }

    fn delete(&self, _id : impl Into<String>) -> Result<Self::TMF,crate::common::tmf_error::TMFError> {
        Err(TMFError::from("Not implemented"))    
    }

    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,crate::common::tmf_error::TMFError> {
        get_tmf(self.host.clone(),id.into())    
    }

    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,crate::common::tmf_error::TMFError> {
        list_tmf(self.host.clone(),filter)   
    }

    fn update(&self, _id : impl Into<String>, _patch : Self::TMF) -> Result<Self::TMF,crate::common::tmf_error::TMFError> {
        Err(TMFError::from("Not implemented"))    
    }
}

impl TMF622ProductOrder {
    /// Create a new instance of Product Order API Object
    pub fn new(host : Uri) -> TMF622ProductOrder {
        TMF622ProductOrder{ host }
    }
}

/// Product Ordering API
#[derive(Clone,Default,Debug)]
pub struct TMF622 {
    host : Uri,
}

impl TMF622 {
    /// Create a new instance of TMF622 API
    pub fn new(host : Uri) -> TMF622 {
        TMF622 {
            host
        }
    }

    /// Access the order module of TMF622.
    pub fn order(&self) -> TMF622ProductOrder {
        TMF622ProductOrder::new(self.host.clone())
    }
}

