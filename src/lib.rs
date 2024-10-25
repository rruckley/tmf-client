
// Copyright [2024] [Ryan Ruckley]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! TMF Client
//! # Description
//! This library provides a client into TMF APIs leveraging tmflib for schema definitons.

#![warn(missing_docs)]

pub mod tmf;
pub mod common;

use common::tmf_error::TMFError;
use tmf::tmf620::TMF620;
use tmf::tmf622::TMF622;
use tmf::tmf629::TMF629;
use tmf::tmf632::TMF632;

use tmflib::HasId;

#[cfg(feature = "oauth2")]
use oauth2::*;

/// Fields for filtering output
#[derive(Clone, Default, Debug)]
pub struct QueryOptions {
    /// Specific set of fields delimited by comma
    pub fields : Option<String>,
    /// Number of records to return
    pub limit : Option<u16>,
    /// Offset for first record in returned results
    pub offset : Option<u16>,
    /// Simple filter on name field, if it exists.
    pub name : Option<String>,
}

impl QueryOptions {
    /// Set the fields to return as a comma separated list.
    pub fn fields(mut self, fields : String) -> QueryOptions {
        self.fields = Some(fields);
        self
    }
    /// Set the number of records to return
    pub fn limit(mut self, limit : u16) -> QueryOptions {
        self.limit = Some(limit);
        self
    }
    /// Set the offset for the returned results, i.e. number of records to skip.
    pub fn offset(mut self, offset : u16) -> QueryOptions {
        self.offset = Some(offset);
        self
    }
    /// Set simple filter on value of name field if exists.
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

/// Standard REST aligned operations supported by all TMF APIs
pub trait Operations {
    /// The concrete TMF schema type
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
    /// Update an existing TMF object
    /// ```
    /// # use tmf_client::{TMFClient,Operations};
    /// use tmflib::tmf620::category::Category;
    /// use tmflib::{HasId,HasName};
    /// let mut existing = Category::new("My Category");
    /// existing.set_name("New Category Name");
    /// let result = TMFClient::new("http://localhost:8080")
    ///     .tmf620()
    ///     .category()
    ///     .update(existing.get_id(),existing);
    /// ```
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

/// TMFClient for interacting with TMF APIs.
/// ```
/// use tmf_client::{TMFClient,Operations,QueryOptions};
/// let filter = QueryOptions::default().limit(5);
/// let specifications = TMFClient::new("http://localhost:8000")
///     .tmf620()
///     .product_specification()
///     .list(Some(filter));
/// ```
pub struct TMFClient {
    host : String,
    tmf620 : Option<TMF620>,
    tmf622 : Option<TMF622>,
    tmf629 : Option<TMF629>,
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
            tmf629 : None,
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
    ///     .tmf629();
    /// ```
    pub fn tmf629(&mut self) -> TMF629 {
        match self.tmf629.as_mut() {
            Some(tmf) => tmf.clone(),
            None => {
                let tmf = TMF629::new(self.host.clone());
                self.tmf629 = Some(tmf.clone());
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