//! TMF632 Party
//!

use tmflib::tmf632::organization_v4::Organization;

use super::{create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf};
use crate::common::tmf_error::TMFError;
#[cfg(not(feature = "blocking"))]
use crate::AsyncOperations;
#[cfg(feature = "blocking")]
use crate::BlockingOperations;
use crate::{Config, HasNew};
#[cfg(feature = "v4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "v5")]
use tmflib::tmf632::individual_v5::Individual;

/// TMF622 Product Order Object
pub struct TMF632Individual {
    /// End-point for TMF622
    config: Config,
}

/// TMF632 Organization
pub struct TMF632Organization {
    config: Config,
}

impl TMF632Individual {
    /// Create a new instance of Product Order API Object
    pub fn new(config: Config) -> TMF632Individual {
        TMF632Individual { config }
    }
}

#[cfg(feature = "blocking")]
impl BlockingOperations for TMF632Individual {
    type TMF = Individual;

    fn create(&self, item: Self::TMF) -> Result<Self::TMF, TMFError> {
        create_tmf(&self.config, item)
    }
    fn delete(&self, id: impl Into<String>) -> Result<Self::TMF, TMFError> {
        delete_tmf(&self.config, id.into())
    }
    fn get(&self, id: impl Into<String>) -> Result<Vec<Self::TMF>, TMFError> {
        get_tmf(&self.config, id.into())
    }
    fn list(&self, filter: Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>, TMFError> {
        list_tmf(&self.config, filter)
    }
    fn update(&self, id: impl Into<String>, patch: Self::TMF) -> Result<Self::TMF, TMFError> {
        update_tmf(&self.config, id.into(), patch)
    }
}

#[cfg(not(feature = "blocking"))]
impl AsyncOperations for TMF632Individual {
    type TMF = Individual;

    async fn create(&self, item: Self::TMF) -> Result<Self::TMF, TMFError> {
        create_tmf(&self.config, item).await
    }
    async fn delete(&self, id: impl Into<String>) -> Result<Self::TMF, TMFError> {
        delete_tmf(&self.config, id.into()).await
    }
    async fn get(&self, id: impl Into<String>) -> Result<Vec<Self::TMF>, TMFError> {
        get_tmf(&self.config, id.into()).await
    }
    async fn list(&self, filter: Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>, TMFError> {
        list_tmf(&self.config, filter).await
    }
    async fn update(&self, id: impl Into<String>, patch: Self::TMF) -> Result<Self::TMF, TMFError> {
        update_tmf(&self.config, id.into(), patch).await
    }
}

impl TMF632Organization {
    /// Create a new instance of Product Order API Object
    pub fn new(config: Config) -> TMF632Organization {
        TMF632Organization { config }
    }
}

#[cfg(feature = "blocking")]
impl BlockingOperations for TMF632Organization {
    type TMF = Organization;

    fn create(&self, item: Self::TMF) -> Result<Self::TMF, TMFError> {
        create_tmf(&self.config, item)
    }
    fn delete(&self, id: impl Into<String>) -> Result<Self::TMF, TMFError> {
        delete_tmf(&self.config, id.into())
    }
    fn get(&self, id: impl Into<String>) -> Result<Vec<Self::TMF>, TMFError> {
        get_tmf(&self.config, id.into())
    }
    fn list(&self, filter: Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>, TMFError> {
        list_tmf(&self.config, filter)
    }
    fn update(&self, id: impl Into<String>, patch: Self::TMF) -> Result<Self::TMF, TMFError> {
        update_tmf(&self.config, id.into(), patch)
    }
}

#[cfg(not(feature = "blocking"))]
impl AsyncOperations for TMF632Organization {
    type TMF = Organization;

    async fn create(&self, item: Self::TMF) -> Result<Self::TMF, TMFError> {
        create_tmf(&self.config, item).await
    }
    async fn delete(&self, id: impl Into<String>) -> Result<Self::TMF, TMFError> {
        delete_tmf(&self.config, id.into()).await
    }
    async fn get(&self, id: impl Into<String>) -> Result<Vec<Self::TMF>, TMFError> {
        get_tmf(&self.config, id.into()).await
    }
    async fn list(&self, filter: Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>, TMFError> {
        list_tmf(&self.config, filter).await
    }
    async fn update(&self, id: impl Into<String>, patch: Self::TMF) -> Result<Self::TMF, TMFError> {
        update_tmf(&self.config, id.into(), patch).await
    }
}

/// Product Ordering API
#[derive(Clone, Debug)]
pub struct TMF632 {
    config: Config,
}

impl HasNew<TMF632> for TMF632 {
    fn new(config: Config) -> TMF632 {
        TMF632 { config }
    }
}

impl TMF632 {
    /// Access the order module of TMF622.
    pub fn individual(&self) -> TMF632Individual {
        TMF632Individual::new(self.config.clone())
    }

    /// Create a new Organisation API access object
    pub fn organization(&self) -> TMF632Organization {
        TMF632Organization::new(self.config.clone())
    }
}
