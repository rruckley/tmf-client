//! TMF Modules
//! 

use crate::QueryOptions;
use crate::common::tmf_error::TMFError;
use tmflib::{HasId,HasName,Uri};
use serde::de::DeserializeOwned;

pub mod tmf620;
pub mod tmf622;

/// Make API call to retrieve a single TMF object
pub fn get_tmf<T : HasId + HasName + DeserializeOwned>(host: Uri, id : String) -> Result<Vec<T>,TMFError> {
    // Return results
    let url = format!("{}{}/{}",host,T::get_class_href(),id);
    let objects = reqwest::blocking::get(url)?.text()?;
    let output : Vec<T> = serde_json::from_str(objects.as_str()).unwrap();
    Ok(output)
}

/// Make API call to retrieve a set of TMF objects according to filter
pub fn list_tmf<T : HasId + HasName + DeserializeOwned>(host: Uri, filter : Option<QueryOptions>) -> Result<Vec<T>,TMFError> {
    // Return results
    let filter = match filter {
        Some(f) => f.into(),
        None => String::default(),
    };
    let url = format!("{}{}?{}",host,T::get_class_href(),filter);
    println!("Filter: {}",filter);
    let objects = reqwest::blocking::get(url)?.text()?;
    let output : Vec<T> = serde_json::from_str(objects.as_str())?;
    Ok(output)
}

/// Create a new TMF object
pub fn create_tmf<T : HasId + DeserializeOwned>(host : Uri, _item : T) -> Result<T,TMFError> {
    let _url = format!("{}{}",host,T::get_class_href());
    Err(TMFError::from("Not implemented yet"))
}

/// Update an existing TMF object
pub fn update_tmf<T : HasId + DeserializeOwned>(host : Uri, id : impl Into<String>, _patch : T) -> Result<T,TMFError> {
    let _url = format!("{}{}/{}",host,T::get_class_href(),id.into());
    //let object = reqwest::blocking::ClientBuilder:
    Err(TMFError::from("Not implemented yet"))
}

/// Delete an existing TMF object
pub fn delete_tmf<T : HasId>(host : Uri, id : impl Into<String>) -> Result<T,TMFError> {
    let _url = format!("{}{}/{}",host,T::get_class_href(),id.into());
    Err(TMFError::from("Not implemented yet"))
}