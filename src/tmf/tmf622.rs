//! TMF622 Product Order
use crate::{Config, HasNew, Operations};
use tmflib::tmf622::product_order_v4::ProductOrder;

use super::{create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf};

/// TMF622 Product Order Object
pub struct TMF622ProductOrder {
    /// End-point for TMF622
    config: Config,
}

impl Operations for TMF622ProductOrder {
    type TMF = ProductOrder;

    fn create(&self, item: Self::TMF) -> Result<Self::TMF, crate::common::tmf_error::TMFError> {
        create_tmf(&self.config, item)
    }

    fn delete(
        &self,
        id: impl Into<String>,
    ) -> Result<Self::TMF, crate::common::tmf_error::TMFError> {
        delete_tmf(&self.config, id.into())
    }

    fn get(
        &self,
        id: impl Into<String>,
    ) -> Result<Vec<Self::TMF>, crate::common::tmf_error::TMFError> {
        get_tmf(&self.config, id.into())
    }

    fn list(
        &self,
        filter: Option<crate::QueryOptions>,
    ) -> Result<Vec<Self::TMF>, crate::common::tmf_error::TMFError> {
        list_tmf(&self.config, filter)
    }

    fn update(
        &self,
        id: impl Into<String>,
        patch: Self::TMF,
    ) -> Result<Self::TMF, crate::common::tmf_error::TMFError> {
        update_tmf(&self.config, id.into(), patch)
    }
}

impl TMF622ProductOrder {
    /// Create a new instance of Product Order API Object
    pub fn new(config: Config) -> TMF622ProductOrder {
        TMF622ProductOrder { config }
    }
}

/// Product Ordering API
#[derive(Clone, Debug)]
pub struct TMF622 {
    config: Config,
}

impl HasNew<TMF622> for TMF622 {
    fn new(config: Config) -> TMF622 {
        TMF622 { config }
    }
}

impl TMF622 {
    /// Access the order module of TMF622.
    pub fn order(&self) -> TMF622ProductOrder {
        TMF622ProductOrder::new(self.config.clone())
    }
}
