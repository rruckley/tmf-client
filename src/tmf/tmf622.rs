//! TMF622 Product Order
use tmflib::{tmf622::product_order_v4::ProductOrder, Uri};
use crate::{Operations,Config,HasNew};

use super::{
    get_tmf,
    list_tmf,
    create_tmf,
    update_tmf,
    delete_tmf
};

/// TMF622 Product Order Object
pub struct TMF622ProductOrder {
    /// End-point for TMF622
    config : &'static Config,
}

impl Operations for TMF622ProductOrder {
    type TMF = ProductOrder;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,crate::common::tmf_error::TMFError> {
        create_tmf(self.host.clone(), item)    
    }

    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,crate::common::tmf_error::TMFError> {
        delete_tmf(self.host.clone(),id.into())
    }

    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,crate::common::tmf_error::TMFError> {
        get_tmf(self.host.clone(),id.into())    
    }

    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,crate::common::tmf_error::TMFError> {
        list_tmf(self.host.clone(),filter)   
    }

    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,crate::common::tmf_error::TMFError> {
        update_tmf(self.host.clone(),id.into(), patch)
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

impl HasNew<TMF622> for TMF622 {
    fn new(config : &Config) -> TMF622 {
        TMF622 {
            config
        }      
    }
}

impl TMF622 {
    /// Access the order module of TMF622.
    pub fn order(&self) -> TMF622ProductOrder {
        TMF622ProductOrder::new(self.host.clone())
    }
}

