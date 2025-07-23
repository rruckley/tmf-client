//! TMF663 Shopping Cart API

//!! This module provides access to the TMF663 Shopping Cart API, allowing
//! for operations such as creating, retrieving, and managing shopping carts.
//! It is designed to be used with the TMFClient, which provides a convenient
//! interface for interacting with the API.
    

use tmflib::Uri;
use tmflib::tmf663::shopping_cart::ShoppingCart;

use crate::common::tmf_error::TMFError;
use crate::{Operations,HasNew};
use super::{
    create_tmf, delete_tmf, get_tmf, list_tmf, update_tmf
};

/// TMF663 Shopping Cart API Object
#[derive(Clone,Default,Debug)]
pub struct TMF663ShoppingCart {
    host : Uri,
}

/// Operations trait implementation for TMF663ShoppingCart
#[derive(Clone,Default,Debug)]
pub struct TMF663 {
    host: Uri,
}

impl HasNew<TMF663> for TMF663 {
    fn new(host : Uri) -> TMF663 {
        TMF663 {
            host
        }
    }
}

impl TMF663 {
    /// Provide a ShoppingCart API object
    /// This method returns an instance of TMF663ShoppingCart, which can be used
    /// to perform operations related to shopping carts.
    /// ```
    /// # use tmf_client::TMFClient;
    /// let shopping_cart = TMFClient::new("http://localhost:8000")
    ///     .tmf663()
    ///     .shopping_cart();
    /// ```
    pub fn shopping_cart(&self) -> TMF663ShoppingCart {
        TMF663ShoppingCart {
            host: self.host.clone(),
        }
    }
}

impl Operations for TMF663ShoppingCart {
    type TMF = ShoppingCart;
    
    fn create(&self, item : Self::TMF) -> Result<Self::TMF,TMFError> {
        create_tmf(self.host.clone(),item)    
    }
    fn delete(&self, id : impl Into<String>) -> Result<Self::TMF,TMFError> {
        delete_tmf::<ShoppingCart>(self.host.clone(), id) 
    }
    fn get(&self, id : impl Into<String>) -> Result<Vec<Self::TMF>,TMFError> {
        get_tmf(self.host.clone(),id.into())  
    }
    fn list(&self, filter : Option<crate::QueryOptions>) -> Result<Vec<Self::TMF>,TMFError> {
        list_tmf(self.host.clone(),filter)  
    }
    fn update(&self, id : impl Into<String>, patch : Self::TMF) -> Result<Self::TMF,TMFError> {
        update_tmf(self.host.clone(), id, patch) 
    }
}