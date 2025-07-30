//! TMF633 Module

use tmflib::tmf633::service_candidate::ServiceCandidate;
use tmflib::tmf633::service_specification::ServiceSpecification;
use tmflib::tmf633::service_catalog::ServiceCatalog;
use tmflib::tmf633::service_category::ServiceCategory;

use crate::common::tmf_error::TMFError;
use crate::{Operations,HasNew,Config};
use super::{
    create_tmf,
    get_tmf,
    list_tmf,
    update_tmf,
    delete_tmf,
};

/// ServiceCandidate API Object
pub struct TMF633Candidate {
    config: Config,
}

/// ServiceCatalog API Object
pub struct TMF633Catalog {
    config: Config,
}

/// ServiceCategory API Object
pub struct TMF633Category {
    config: Config,
}

/// ServiceSpecification API Object
pub struct TMF633Specification {
    config: Config,
}

impl Operations for TMF633Candidate {
    type TMF = ServiceCandidate;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(&self.config,item)    
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf(&self.config, id.into())
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(&self.config,id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(&self.config,filter)  
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(&self.config, id, patch) 
    }    
}

impl Operations for TMF633Catalog {
    type TMF = ServiceCatalog;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(&self.config,item)    
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf(&self.config, id.into()) 
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(&self.config,id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(&self.config,filter)  
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(&self.config, id, patch) 
    }
}

impl Operations for TMF633Category {
    type TMF = ServiceCategory;
          
    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(&self.config,item)    
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf(&self.config, id) 
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(&self.config,id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(&self.config,filter)  
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(&self.config, id, patch) 
    } 
}

impl Operations for TMF633Specification {
    type TMF = ServiceSpecification;

    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(&self.config,item)    
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf(&self.config, id)
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(&self.config,id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(&self.config,filter)  
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(&self.config, id, patch) 
    }        
}

/// Product Catalogue API
#[derive(Clone,Debug)]
pub struct TMF633 {
    config : Config,
}

impl HasNew<TMF633> for TMF633 {
    fn new(config : Config) -> TMF633 {
        TMF633 {
            config
        }
    }
}

impl TMF633 {
    /// Create ServiceCandidate API object
    pub fn candidate(&self) -> TMF633Candidate {
        TMF633Candidate { config: self.config.clone() }
    }

    /// Create ServiceCatalog API object
    pub fn catalog(&self) -> TMF633Catalog {
        TMF633Catalog { config: self.config.clone()}
    }

    /// Create ServiceCategory API Object
    pub fn category(&self) -> TMF633Category {
        TMF633Category { config: self.config.clone() }
    }

    /// Create ServiceSpecification API Object
    pub fn specification(&self) -> TMF633Specification {
        TMF633Specification { config: self.config.clone() }
    }
}