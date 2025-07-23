//! # TMF Client Library
//! 
//! ## Description
//! Interact with TMF compliant APIs using an SDK
//! 
//! ## Supported TMF APIs
//! 
//! Currently supports:
//! 
//! - TMF620
//! - TMF622
//! - TMF629
//! - TMF632
//! - TMF633
//! - TMF648
//! - TMF674
//! 
//! ## Features
//! All TMF APIs can be conditionally compiled. Deafult includes all APIs using V4 specifications.

#[warn(missing_docs)]

pub mod tmf;
pub mod common;

use common::tmf_error::TMFError;
#[cfg(feature = "tmf620")]
use tmf::tmf620::TMF620;
#[cfg(feature = "tmf622")]
use tmf::tmf622::TMF622;
#[cfg(feature = "tmf629")]
use tmf::tmf629::TMF629;
#[cfg(feature = "tmf632")]
use tmf::tmf632::TMF632;
#[cfg(feature = "tmf633")]
use tmf::tmf633::TMF633;
#[cfg(feature = "tmf645")]
#[cfg(feature = "tmf637")]
use tmf::tmf637::TMF637;
#[cfg(feature = "tmf638")]
use tmf::tmf638::TMF638;
#[cfg(feature = "tmf639")]
use tmf::tmf639::TMF639;
use tmf::tmf645::TMF645;
#[cfg(feature = "tmf648")]
use tmf::tmf648::TMF648;
#[cfg(feature = "tmf663")]
use tmf::tmf663::TMF663;
#[cfg(feature = "tmf674")]
use tmf::tmf674::TMF674;


use tmflib::{HasId,Uri};

/// Fields for filtering output
#[derive(Clone, Default, Debug)]
pub struct QueryOptions {
    /// Specific set of fields delimited by comma
    pub fields : Option<String>,
    pub limit : Option<u16>,
    pub offset : Option<u16>,
    /// Filter on name
    pub name : Option<String>,
}

impl QueryOptions {
    pub fn fields(mut self, fields : String) -> QueryOptions {
        self.fields = Some(fields);
        self
    }
    pub fn limit(mut self, limit : u16) -> QueryOptions {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset : u16) -> QueryOptions {
        self.offset = Some(offset);
        self
    }

    pub fn name(mut self, name : impl Into<String>) -> QueryOptions {
        self.name = Some(name.into());
        self
    }
}

impl From<QueryOptions> for String {
    fn from(val: QueryOptions) -> Self {
        let limit = match val.limit {
            Some(l) => format!("limit={}",l),
            None => String::default(),
        };
        let offset = match val.offset {
            Some(o) => format!("offset={}",o),
            None => String::default(),
        };
        let name = match val.name {
            Some(n) => format!("name={}",n),
            None => String::default(),
        };
        format!("{}&{}&{}",limit,offset,name)   
    }
}

/// Standard set of operations for all TMF objects
pub trait Operations {
    type TMF : HasId;

    /// Get a specific TMF object by Id
    /// ```
    /// # use tmf_client::{TMFClient,Operations};
    /// let categories = TMFClient::new("http://localhost:8000")
    ///     .tmf620()
    ///     .category()
    ///     .get("ID123");
    /// ```
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError>;
    /// Get a list of tmf objects applying optional filter
    /// ```
    /// # use tmf_client::{TMFClient,QueryOptions,Operations};
    /// let filter = QueryOptions::default()
    ///     .limit(15)
    ///     .offset(10);
    /// let categories = TMFClient::new("http://localhost:8000")
    ///     .tmf620()
    ///     .category()
    ///     .list(Some(filter));
    /// ```
    fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Self::TMF>,TMFError>;
    /// Create a new instance of a TMF object
    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError>;
    /// Update an existing TMF Object using the provided patch object
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError>;
    /// Delete a specific tmf object by Id
    /// ```
    /// # use tmf_client::{TMFClient,Operations};
    /// let categories = TMFClient::new("http://localhost:8000")
    ///     .tmf620()
    ///     .category()
    ///     .delete("ID123");
    /// ```
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError>;
}

pub trait HasNew<T : Clone> {
    fn new(host : Uri) -> T;
}

pub struct TMFClient {
    host : String,
    #[cfg(feature = "tmf620")]
    tmf620 : Option<TMF620>,
    #[cfg(feature = "tmf622")]
    tmf622 : Option<TMF622>,
    #[cfg(feature = "tmf629")]
    tmf629 : Option<TMF629>,
    #[cfg(feature = "tmf632")]
    tmf632 : Option<TMF632>,
    #[cfg(feature = "tmf633")]
    tmf633 : Option<TMF633>,
    #[cfg(feature = "tmf645")]
    tmf645 : Option<TMF645>,
    #[cfg(feature = "tmf648")]
    tmf648 : Option<TMF648>,
    #[cfg(feature = "tmf663")]
    tmf663 : Option<TMF663>,
    #[cfg(feature = "tmf674")]
    tmf674 : Option<TMF674>,
}

// Create a new instance
fn instantiate<T : Clone + HasNew<T>>(tmf : &mut Option<T>,hostname : String) -> T {
    match tmf {
        // If we already have an instance, clone that.
        Some(t) => t.clone(),
        // Else we need to create a new one and also store it.
        None => {
            let new_tmf = T::new(hostname);
            tmf.replace(new_tmf.clone());
            new_tmf
        },
    }
}

impl TMFClient {
    /// Create a new TMFClient instance
    /// ```
    /// # use tmf_client::TMFClient;
    /// let client = TMFClient::new("http://localhost:8000");
    /// ```
    pub fn new(host : impl Into<String>) -> TMFClient {
        TMFClient {
            host : host.into(),
            #[cfg(feature = "tmf620")]
            tmf620 : None,
            #[cfg(feature = "tmf622")]
            tmf622 : None,
            #[cfg(feature = "tmf629")]
            tmf629 : None,
            #[cfg(feature = "tmf632")]
            tmf632 : None,
            #[cfg(feature = "tmf633")]
            tmf633 : None,
            #[cfg(feature = "tmf645")]
            tmf645 : None,
            #[cfg(feature = "tmf648")]
            tmf648:  None,
            #[cfg(feature = "tmf663")]
            tmf663:  None,
            #[cfg(feature = "tmf674")]
            tmf674 : None,
        }
    }



    /// Create access to TMF620 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf620 = TMFClient::new("http://localhost:8000")
    ///     .tmf620();
    /// ```
    #[cfg(feature = "tmf620")]
    pub fn tmf620(&mut self) -> TMF620 {
        instantiate(&mut self.tmf620,self.host.clone())
    }

    /// Create access to TMF622 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf620 = TMFClient::new("http://localhost:8000")
    ///     .tmf622();
    /// ```
    #[cfg(feature = "tmf622")]
    pub fn tmf622(&mut self) -> TMF622 {
        instantiate(&mut self.tmf622, self.host.clone())
    }

    /// Create access to TMF632 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf632 = TMFClient::new("http://localhost:8000")
    ///     .tmf629();
    /// ```
    #[cfg(feature = "tmf629")]
    pub fn tmf629(&mut self) -> TMF629 {
        instantiate(&mut self.tmf629, self.host.clone())
    }

    /// Create access to TMF632 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf632 = TMFClient::new("http://localhost:8000")
    ///     .tmf632();
    /// ```
    #[cfg(feature = "tmf632")]
    pub fn tmf632(&mut self) -> TMF632 {
        instantiate(&mut self.tmf632, self.host.clone())
    }

    #[cfg(feature = "tmf633")]
    pub fn tmf633(&mut self) -> TMF633 {
        instantiate(&mut self.tmf633, self.host.clone())
    }

    #[cfg(feature = "tmf645")]
    pub fn tmf645(&mut self) -> TMF645 {
        instantiate(&mut self.tmf645, self.host.clone())
    }

    #[cfg(feature = "tmf648")]
    pub fn tmf648(&mut self) -> TMF648 {
        instantiate(&mut self.tmf648, self.host.clone())
    }

    /// Create access to TMF663 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf663 = TMFClient::new("http://localhost:8000")
    ///     .tmf663();
    /// ```
    #[cfg(feature = "tmf663")]
    pub fn tmf663(&mut self) -> TMF663 {
        instantiate(&mut self.tmf663, self.host.clone())
    }

    /// Create access to TMF674 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf674 = TMFClient::new("http://localhost:8000")
    ///     .tmf674();
    /// ```
    #[cfg(feature = "tmf674")]
    pub fn tmf674(&mut self) -> TMF674 {
        instantiate(&mut self.tmf674, self.host.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_filter_limit() {
        let filter = QueryOptions::default().limit(111);

        assert_eq!(filter.limit,Some(111));
    }

    #[test]
    fn test_filter_offset() {
        let filter = QueryOptions::default().offset(222);

        assert_eq!(filter.offset,Some(222));
    }
}