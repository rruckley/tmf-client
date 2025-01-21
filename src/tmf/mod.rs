//! TMF Modules
//! 

use std::io::Read;

use crate::QueryOptions;
use crate::common::tmf_error::TMFError;
use tmflib::{HasId,Uri};
use serde::{de::DeserializeOwned, Serialize};
// use log::info;

#[cfg(feature = "tmf620")]
pub mod tmf620;
#[cfg(feature = "tmf622")]
pub mod tmf622;
#[cfg(feature = "tmf629")]
pub mod tmf629;
#[cfg(feature = "tmf632")]
pub mod tmf632;
#[cfg(feature = "tmf633")]
pub mod tmf633;
#[cfg(feature = "tmf648")]
pub mod tmf648;
#[cfg(feature = "tmf674")]
pub mod tmf674;

/// Make API call to retrieve a single TMF object
pub fn get_tmf<T : HasId + DeserializeOwned>(host: Uri, id : String) -> Result<Vec<T>,TMFError> {
    // Return results
    let url = format!("{}{}/{}",host,T::get_class_href(),id);
    // let objects = reqwest::blocking::get(url)?.text()?;
    let client = reqwest::blocking::Client::new();    
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    let agent = format!("{}/{}",pkg,ver);
    let objects = client
        .get(url)
        .header("User-Agent", agent)
        .send()?
        .text()?;
    let output : T = serde_json::from_str(objects.as_str())?;
    Ok(vec![output])
}

/// Make API call to retrieve a set of TMF objects according to filter
pub fn list_tmf<T : HasId + DeserializeOwned>(host: Uri, filter : Option<QueryOptions>) -> Result<Vec<T>,TMFError> {
    // Return results
    let filter = match filter {
        Some(f) => f.into(),
        None => String::default(),
    };
    let url = format!("{}{}?{}",host,T::get_class_href(),filter);
    // info!("Filter: {}",filter);
    let client = reqwest::blocking::Client::new();    
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    let agent = format!("{}/{}",pkg,ver);
    let objects = client
        .get(url)
        .header("User-Agent", agent)
        .send()?
        .text()?;
    let output : Vec<T> = serde_json::from_str(objects.as_str())?;
    Ok(output)
}

/// Create a new TMF object
pub fn create_tmf<T : HasId + Serialize + DeserializeOwned>(host : Uri, item : T) -> Result<T,TMFError> {
    let url = format!("{}{}",host,T::get_class_href());
    let client = reqwest::blocking::Client::new();
    let body_str = serde_json::to_string(&item)?;
    let mut res = client.post(url)
        .body(body_str)
        .send()?;
    let mut output = String::default();
    let _ = res.read_to_string(&mut output);
    let item : T = serde_json::from_str(output.as_str())?;
    Ok(item)
}

/// Update an existing TMF object
pub fn update_tmf<T : HasId + Serialize + DeserializeOwned>(host : Uri, id : impl Into<String>, patch : T) -> Result<T,TMFError> {
    let url = format!("{}{}/{}",host,T::get_class_href(),id.into());
    let client = reqwest::blocking::Client::new();
    let body_str = serde_json::to_string(&patch)?;
    let mut res = client.patch(url)
        .body(body_str)
        .send()?;
    let mut output = String::default();
    let _ = res.read_to_string(&mut output);
    let item : T = serde_json::from_str(output.as_str())?;
    Ok(item)
}

/// Delete an existing TMF object
pub fn delete_tmf<T : HasId>(host : Uri, id : impl Into<String>) -> Result<T,TMFError> {
    let url = format!("{}{}/{}",host,T::get_class_href(),id.into().clone());
    let client = reqwest::blocking::Client::new();
    let mut _res = client.delete(url)
        .send()?;
    // Return empty object for now to avoid
    // round trip to retrieve object
    let out = T::default();
    // out.set_id(id);
    Ok(out)
}