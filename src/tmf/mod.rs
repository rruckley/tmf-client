//! TMF Modules
//!

#[cfg(feature = "blocking")]
use std::io::Read;

use crate::common::tmf_error::TMFError;
use crate::{Config, QueryOptions};
use serde::{de::DeserializeOwned, Serialize};
use tmflib::HasId;
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
#[cfg(feature = "tmf637")]
pub mod tmf637;
#[cfg(feature = "tmf638")]
pub mod tmf638;
#[cfg(feature = "tmf639")]
pub mod tmf639;
#[cfg(feature = "tmf645")]
pub mod tmf645;
#[cfg(feature = "tmf648")]
pub mod tmf648;
#[cfg(feature = "tmf663")]
pub mod tmf663;
#[cfg(feature = "tmf674")]
pub mod tmf674;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn gen_url(config: &Config) -> String {
    format!("{}:{}/", config.host, config.port)
}

/// Make API call to retrieve a single TMF object
#[cfg(feature = "blocking")]
pub fn get_tmf<T: HasId + DeserializeOwned>(
    config: &Config,
    id: String,
) -> Result<Vec<T>, TMFError> {
    // Return results
    let url = format!("{}{}/{}", gen_url(config), T::get_class_href(), id);

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .user_agent(USER_AGENT)
        .use_rustls_tls()
        .build()?;

    let objects = client.get(url).send()?.text()?;
    let output: T = serde_json::from_str(objects.as_str())?;
    Ok(vec![output])
}

/// Get a single TMF record
#[cfg(not(feature = "blocking"))]
pub async fn get_tmf<T: HasId + DeserializeOwned>(
    config: &Config,
    id: String,
) -> Result<Vec<T>, TMFError> {
    // Return results
    let url = format!("{}{}/{}", gen_url(config), T::get_class_href(), id);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .user_agent(USER_AGENT)
        .use_rustls_tls()
        .build()?;

    let objects = client.get(url).send().await?.text().await?;
    let output: T = serde_json::from_str(objects.as_str())?;
    Ok(vec![output])
}

/// Make API call to retrieve a set of TMF objects according to filter
#[cfg(feature = "blocking")]
pub fn list_tmf<T: HasId + DeserializeOwned>(
    config: &Config,
    filter: Option<QueryOptions>,
) -> Result<Vec<T>, TMFError> {
    // Return results
    let filter = match filter {
        Some(f) => f.into(),
        None => String::default(),
    };
    let url = format!("{}{}?{}", gen_url(config), T::get_class_href(), filter);

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .user_agent(USER_AGENT)
        .use_rustls_tls()
        .build()?;

    let objects = client.get(url).send()?.text()?;
    let output: Vec<T> = serde_json::from_str(objects.as_str())?;
    Ok(output)
}

/// List TMF records , optionally apply a filter
#[cfg(not(feature = "blocking"))]
pub async fn list_tmf<T: HasId + DeserializeOwned>(
    config: &Config,
    filter: Option<QueryOptions>,
) -> Result<Vec<T>, TMFError> {
    // Return results
    let filter = match filter {
        Some(f) => f.into(),
        None => String::default(),
    };
    let url = format!("{}{}?{}", gen_url(config), T::get_class_href(), filter);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .user_agent(USER_AGENT)
        .use_rustls_tls()
        .build()?;

    let objects = client.get(url).send().await?.text().await?;
    let output: Vec<T> = serde_json::from_str(objects.as_str())?;
    Ok(output)
}

/// Create a new TMF object
#[cfg(feature = "blocking")]
pub fn create_tmf<T: HasId + Serialize + DeserializeOwned>(
    config: &Config,
    item: T,
) -> Result<T, TMFError> {
    let url = format!("{}{}", gen_url(config), T::get_class_href());

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .use_rustls_tls()
        .user_agent(USER_AGENT)
        .build()?;
    let body_str = serde_json::to_string(&item)?;
    let mut res = client.post(url).body(body_str).send()?;
    let mut output = String::default();
    let _count = res.read_to_string(&mut output)?;
    match res.status() {
        reqwest::StatusCode::CREATED | reqwest::StatusCode::OK => {
            let item: T = serde_json::from_str(output.as_str())?;
            Ok(item)
        }
        _ => Err(TMFError::Unknown(format!(
            "Failed to create TMF object: {output}"
        ))),
    }
}

/// Create a new TMF record
#[cfg(not(feature = "blocking"))]
pub async fn create_tmf<T: HasId + Serialize + DeserializeOwned>(
    config: &Config,
    item: T,
) -> Result<T, TMFError> {
    let url = format!("{}{}", gen_url(config), T::get_class_href());

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .use_rustls_tls()
        .user_agent(USER_AGENT)
        .build()?;
    let body_str = serde_json::to_string(&item)?;
    let res = client.post(url).body(body_str).send().await?;
    // let mut output = String::default();
    // let _count = res.read_to_string(&mut output)?;
    let status = res.status();
    let output = res.text().await?;
    match status {
        reqwest::StatusCode::CREATED | reqwest::StatusCode::OK => {
            let item: T = serde_json::from_str(output.as_str())?;
            Ok(item)
        }
        _ => Err(TMFError::Unknown(format!(
            "Failed to create TMF object: {output}"
        ))),
    }
}

/// Update an existing TMF object
#[cfg(feature = "blocking")]
pub fn update_tmf<T: HasId + Serialize + DeserializeOwned>(
    config: &Config,
    id: impl Into<String>,
    patch: T,
) -> Result<T, TMFError> {
    let url = format!("{}{}/{}", gen_url(config), T::get_class_href(), id.into());

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .use_rustls_tls()
        .user_agent(USER_AGENT)
        .build()?;

    let body_str = serde_json::to_string(&patch)?;
    let mut res = client.patch(url).body(body_str).send()?;
    let mut output = String::default();
    let _ = res.read_to_string(&mut output);
    let item: T = serde_json::from_str(output.as_str())?;
    Ok(item)
}

/// Update a TMF record
#[cfg(not(feature = "blocking"))]
pub async fn update_tmf<T: HasId + Serialize + DeserializeOwned>(
    config: &Config,
    id: impl Into<String>,
    patch: T,
) -> Result<T, TMFError> {
    let url = format!("{}{}/{}", gen_url(config), T::get_class_href(), id.into());

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .use_rustls_tls()
        .user_agent(USER_AGENT)
        .build()?;

    let body_str = serde_json::to_string(&patch)?;
    let res = client.patch(url).body(body_str).send().await?;
    // let mut output = String::default();
    // let _ = res.read_to_string(&mut output);
    let item: T = serde_json::from_str(res.text().await?.as_str())?;
    Ok(item)
}

/// Delete an existing TMF object
#[cfg(feature = "blocking")]
pub fn delete_tmf<T: HasId>(config: &Config, id: impl Into<String>) -> Result<T, TMFError> {
    let url = format!(
        "{}{}/{}",
        gen_url(config),
        T::get_class_href(),
        id.into().clone()
    );

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .use_rustls_tls()
        .user_agent(USER_AGENT)
        .build()?;

    let mut _res = client.delete(url).send()?;
    // Return empty object for now to avoid
    // round trip to retrieve object
    let out = T::default();
    // out.set_id(id);
    Ok(out)
}

/// Delete a single TMF record, returning deleted record
#[cfg(not(feature = "blocking"))]
pub async fn delete_tmf<T: HasId>(config: &Config, id: impl Into<String>) -> Result<T, TMFError> {
    let url = format!(
        "{}{}/{}",
        gen_url(config),
        T::get_class_href(),
        id.into().clone()
    );

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(config.insecure) // For testing purposes only, do not use in production
        .use_rustls_tls()
        .user_agent(USER_AGENT)
        .build()?;

    let mut _res = client.delete(url).send().await?;
    // Return empty object for now to avoid
    // round trip to retrieve object
    let out = T::default();
    // out.set_id(id);
    Ok(out)
}
