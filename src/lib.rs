//! # TMF Client Library
//! 
//! ## Description
//! Interact with TMF compliant APIs using an SDK
//! 
//! ## Supported TMF APIs
//! 
//! Currently supports:
//! 
//! - TMF620 Product Catalog Management
//! - TMF622 Product Order Management
//! - TMF629 Customer Management
//! - TMF632 Party Management
//! - TMF633 Service Catalog Management
//! - TMF637 Product Inventory Management
//! - TMF638 Service Inventory Management
//! - TMF639 Resource Inventory Management
//! - TMF645 Service Qualification Management
//! - TMF648 Quote Management
//! - TMF663 Shopping Cart Management
//! - TMF674 Geographic Site Management
//! 
//! ## Features
//! All TMF APIs can be conditionally compiled. Deafult includes all APIs using V4 specifications.

#![warn(missing_docs)]

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
#[cfg(feature = "tmf637")]
use tmf::tmf637::TMF637;
#[cfg(feature = "tmf638")]
use tmf::tmf638::TMF638;
#[cfg(feature = "tmf639")]
use tmf::tmf639::TMF639;
#[cfg(feature = "tmf645")]
use tmf::tmf645::TMF645;
#[cfg(feature = "tmf648")]
use tmf::tmf648::TMF648;
#[cfg(feature = "tmf663")]
use tmf::tmf663::TMF663;
#[cfg(feature = "tmf674")]
use tmf::tmf674::TMF674;


use tmflib::{HasId,Uri};

#[cfg(feature = "insecure")]
const INSECURE: bool = true;
#[cfg(not(feature = "insecure"))]
const INSECURE: bool = false;

/// Default port for the TMF API
pub const DEFAULT_PORT: u16 = 8001;

/// Configuration for the TMF Client
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// The host URI for the TMF API  
    pub host : Uri,
    /// The port to use for the TMF API 
    /// 
    pub port : u16,
    /// The base path for the TMF API
    pub insecure : bool,
}

impl Config {
    /// Create a new configuration for the TMF Client
    /// ```
    /// # use tmf_client::Config;
    /// let config = Config::new("http://localhost:8000", 8000);
    /// ```
    pub fn new(host : impl Into<String>, port : u16) -> Config {
        Config {
            host : Uri::from(host.into()),
            port,
            insecure : INSECURE,
        }
    }
}

/// Fields for filtering output
#[derive(Clone, Default, Debug)]
pub struct QueryOptions {
    /// Specific set of fields delimited by comma
    pub fields : Option<String>,
    /// Limit the number of results returned
    pub limit : Option<u16>,
    /// Offset the results returned
    pub offset : Option<u16>,
    /// Filter on name
    pub name : Option<String>,
}

impl QueryOptions {
    /// Setting the field on an existing QueryOptions
    /// ```
    /// # use tmf_client::QueryOptions;
    /// let opt = QueryOptions::default()
    ///     .fields("id,name,description".to_string());
    /// ```
    pub fn fields(mut self, fields : String) -> QueryOptions {
        self.fields = Some(fields);
        self
    }
    /// Set the limit on the number of results returned
    /// ```
    /// # use tmf_client::QueryOptions;
    /// let opt = QueryOptions::default()
    ///     .limit(10);
    /// ```
    pub fn limit(mut self, limit : u16) -> QueryOptions {
        self.limit = Some(limit);
        self
    }

    /// Set the offset on the number of results returned
    /// ```
    /// # use tmf_client::QueryOptions;
    /// let opt = QueryOptions::default()
    ///     .offset(5);
    /// ```
    pub fn offset(mut self, offset : u16) -> QueryOptions {
        self.offset = Some(offset);
        self
    }

    /// Set the name to filter on
    /// ```
    /// # use tmf_client::QueryOptions;
    /// let opt = QueryOptions::default()
    ///     .name("MyService".to_string());
    /// ```
    /// This will filter the results to only include those with the specified name.
    /// If the name is not set, it will not filter on name.
    ///
    pub fn name(mut self, name : impl Into<String>) -> QueryOptions {
        self.name = Some(name.into());
        self
    }
}

impl From<QueryOptions> for String {
    fn from(val: QueryOptions) -> Self {
        let limit = match val.limit {
            Some(l) => format!("limit={l}"),
            None => String::default(),
        };
        let offset = match val.offset {
            Some(o) => format!("offset={o}"),
            None => String::default(),
        };
        let name = match val.name {
            Some(n) => format!("name={n}"),
            None => String::default(),
        };
        format!("{limit}&{offset}&{name}")   
    }
}

/// Standard set of operations for all TMF objects
pub trait Operations {
    /// The TMF object type that this trait operates on
    type TMF : HasId;

    /// Get a specific TMF object by Id
    /// ```
    /// # use tmf_client::{TMFClient,Operations};
    /// let categories = TMFClient::new("http://localhost:8000",None)
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
    /// let categories = TMFClient::new("http://localhost:8000",None)
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
    /// let categories = TMFClient::new("http://localhost:8000",None)
    ///     .tmf620()
    ///     .category()
    ///     .delete("ID123");
    /// ```
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError>;
}

/// Trait to create a new instance of a TMF object
#[allow(clippy::new_ret_no_self)]
pub trait HasNew<T : Clone> {
    /// Create a new instance of the TMF object passin in the destination host Uri
    fn new(config : Config) -> T;
}

/// TMF Client
pub struct TMFClient {
    config : Config,
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
    #[cfg(feature = "tmf637")]
    tmf637 : Option<TMF637>,
    #[cfg(feature = "tmf638")]
    tmf638 : Option<TMF638>,
    #[cfg(feature = "tmf639")]
    tmf639 : Option<TMF639>,
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
fn instantiate<T : Clone + HasNew<T>>(api : &mut Option<T>, config : Config) -> T {
    match api {
        Some(instance) => instance.clone(),
        None => {
            let new_api = T::new(config);
            api.replace(new_api.clone());
            new_api
        },
    }
}

impl TMFClient {
    /// Create a new TMFClient instance
    /// ```
    /// # use tmf_client::TMFClient;
    /// let client = TMFClient::new("http://localhost:8000",None);
    /// ```
    pub fn new(host : impl Into<String>, port : Option<u16>) -> TMFClient {
        TMFClient {
            config : Config::new(host,port.unwrap_or(DEFAULT_PORT)),
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
            #[cfg(feature = "tmf637")]
            tmf637 : None,
            #[cfg(feature = "tmf638")]
            tmf638 : None,
            #[cfg(feature = "tmf639")]
            tmf639 : None,
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
    /// let tmf620 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf620();
    /// ```
    #[cfg(feature = "tmf620")]
    pub fn tmf620(&mut self) -> TMF620 {
        // // let instance = 
        let config = self.config.clone();
        let instance : TMF620 = instantiate(&mut self.tmf620,config);
        self.tmf620.replace(instance.clone());
        instance
    }

    /// Create access to TMF622 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf620 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf622();
    /// ```
    #[cfg(feature = "tmf622")]
    pub fn tmf622(&mut self) -> TMF622 {
        instantiate(&mut self.tmf622,self.config.clone())
    }

    /// Create access to TMF632 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf632 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf629();
    /// ```
    #[cfg(feature = "tmf629")]
    pub fn tmf629(&mut self) -> TMF629 {
        instantiate(&mut self.tmf629, self.config.clone())
    }

    /// Create access to TMF632 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf632 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf632();
    /// ```
    #[cfg(feature = "tmf632")]
    pub fn tmf632(&mut self) -> TMF632 {
        instantiate(&mut self.tmf632, self.config.clone())
    }

    /// Create access to TMF633 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf633 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf633();
    /// ```
    #[cfg(feature = "tmf633")]
    pub fn tmf633(&mut self) -> TMF633 {
        instantiate(&mut self.tmf633, self.config.clone())
    }

    /// Create access to TMF637 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf637 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf637();
    /// ```
    #[cfg(feature = "tmf637")]
    pub fn tmf637(&mut self) -> TMF637 {
        instantiate(&mut self.tmf637, self.config.clone())
    }

    /// Create access to TMF638 API
    /// ```
    /// # use tmf_client::TMFClient;                
    /// let tmf638 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf638();
    /// ```
    #[cfg(feature = "tmf638")]
    pub fn tmf638(&mut self) -> TMF638 {
        instantiate(&mut self.tmf638, self.config.clone())
    }

    /// Create access to TMF639 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf639 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf639();
    /// ```
    #[cfg(feature = "tmf639")]
    pub fn tmf639(&mut self) -> TMF639 {
        instantiate(&mut self.tmf639, self.config.clone())
    }

    /// Create access to TMF645 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf645 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf645();
    /// ```
    #[cfg(feature = "tmf645")]
    pub fn tmf645(&mut self) -> TMF645 {
        instantiate(&mut self.tmf645, self.config.clone())
    }

    /// Create access to TMF648 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf648 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf648();
    /// ```
    #[cfg(feature = "tmf648")]
    pub fn tmf648(&mut self) -> TMF648 {
        instantiate(&mut self.tmf648, self.config.clone())
    }

    /// Create access to TMF663 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf663 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf663();
    /// ```
    #[cfg(feature = "tmf663")]
    pub fn tmf663(&mut self) -> TMF663 {
        instantiate(&mut self.tmf663, self.config.clone())
    }

    /// Create access to TMF674 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf674 = TMFClient::new("http://localhost:8000",None)
    ///     .tmf674();
    /// ```
    #[cfg(feature = "tmf674")]
    pub fn tmf674(&mut self) -> TMF674 {
        instantiate(&mut self.tmf674, self.config.clone())
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