//! TMF Modules
//! 

use crate::QueryOptions;
use crate::common::tmf_error::TMFError;
use tmflib::{HasId,HasName,Uri};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod tmf620;
pub mod tmf622;

/// Make API call to retrieve a single TMF object
pub fn get_tmf<T : HasId + HasName + DeserializeOwned>(host: Uri, id : String) -> Result<Vec<T>,TMFError> {
    // Return results
    let url = format!("{}{}",host,T::get_class_href());
    let catalogs = reqwest::blocking::get(url)?.text()?;
    let output : Vec<T> = serde_json::from_str(catalogs.as_str()).unwrap();
    Ok(output)
}

/// Make API call to retrieve a set of TMF objects according to filter
pub fn list_tmf<T : HasId + HasName + DeserializeOwned>(host: Uri, filter : Option<QueryOptions>) -> Result<Vec<T>,TMFError> {
    // Return results
    let filter : String = filter.unwrap().into();
    let url = format!("{}{}?{}",host,T::get_class_href(),filter);
    println!("Filter: {}",filter);
    let catalogs = reqwest::blocking::get(url)?.text()?;
    let output : Vec<T> = serde_json::from_str(catalogs.as_str()).unwrap();
    Ok(output)
}