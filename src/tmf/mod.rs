//! TMF Modules
//! 

use std::io::Read;

use crate::QueryOptions;
use crate::common::tmf_error::TMFError;
use tmflib::{HasId,Uri};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(feature = "oauth2")]
use oauth2::basic::BasicClient;

#[cfg(feature = "oauth2")]
use oauth2::{
    ClientId,
    ClientSecret,
    AuthUrl,
    TokenUrl
};
pub mod tmf620;
pub mod tmf622;
pub mod tmf629;
pub mod tmf632;

/// Get a new OAuth toekn
#[cfg(feature = "oauth2")]
pub fn get_token() {
    let _client = BasicClient::new(
        ClientId::new(std::env::var("OAUTH_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(std::env::var("OAUTH_CLIENT_SECRET").unwrap())),
        AuthUrl::new(std::env::var("OAUTH_AUTH_URL").unwrap()).unwrap(),
        Some(TokenUrl::new(std::env::var("OAUTH_TOKEN_URL").unwrap()).unwrap())
    );
}

/// Make API call to retrieve a single TMF object
pub fn get_tmf<T : HasId + DeserializeOwned>(host: Uri, id : String) -> Result<Vec<T>,TMFError> {
    // Return results
    let url = format!("{}{}/{}",host,T::get_class_href(),id);
    let objects = reqwest::blocking::get(url)?.text()?;
    let output : Vec<T> = serde_json::from_str(objects.as_str())?;
    Ok(output)
    // Err(TMFError::from("Converting to OAuth support"))
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
    let objects = reqwest::blocking::get(url)?.text()?;
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