
#[warn(missing_docs)]

pub mod tmf;
pub mod common;

use common::tmf_error::TMFError;
use tmf::tmf620::TMF620;
use tmf::tmf622::TMF622;
use tmf::tmf632::TMF632;

use tmflib::HasId;

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

pub trait Operations {
    type TMF : HasId;

    /// Get a specific tmf object by Id
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
    /// # use tmf_client::{TMFClient,Operations};
    /// let categories = TMFClient::new("http://localhost:8000")
    ///     .tmf620()
    ///     .category()
    ///     .list(None);
    /// ```
    fn list(&self, filter : Option<QueryOptions>) -> Result<Vec<Self::TMF>,TMFError>;
    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError>;
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

pub struct TMFClient {
    host : String,
    tmf620 : Option<TMF620>,
    tmf622 : Option<TMF622>,
    tmf632 : Option<TMF632>,
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
            tmf620 : None,
            tmf622 : None,
            tmf632 : None,
        }
    }

    /// Create access to TMF620 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf620 = TMFClient::new("http://localhost:8000")
    ///     .tmf620();
    /// ```
    pub fn tmf620(&mut self) -> TMF620 {
        match self.tmf620.as_mut() {
            Some(tmf) => tmf.clone(),
            None => {
                // Allocate a new instance
                let tmf = TMF620::new(self.host.clone());
                self.tmf620 = Some(tmf.clone());
                tmf
            }
        }
    }

    /// Create access to TMF622 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf620 = TMFClient::new("http://localhost:8000")
    ///     .tmf622();
    /// ```
    pub fn tmf622(&mut self) -> TMF622 {
        match self.tmf622.as_mut() {
            Some(tmf) => tmf.clone(),
            None => {
                // Allocate a new instance
                let tmf = TMF622::new(self.host.clone());
                self.tmf622 = Some(tmf.clone());
                tmf
            }
        }
    }

    /// Create access to TMF632 API
    /// ```
    /// # use tmf_client::TMFClient;
    /// let tmf632 = TMFClient::new("http://localhost:8000")
    ///     .tmf632();
    /// ```
    pub fn tmf632(&mut self) -> TMF632 {
        match self.tmf632.as_mut() {
            Some(tmf) => tmf.clone(),
            None => {
                let tmf = TMF632::new(self.host.clone());
                self.tmf632 = Some(tmf.clone());
                tmf
            }
        }
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