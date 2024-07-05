//! Get Customer Exfample

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, TMFClient};
use tmflib::{HasId,HasName};

fn main() -> Result<(),TMFError> {

    let clients = TMFClient::new("http://localhost:8000")
        .tmf629()
        .customer()
        .list(None)?;

    for c in clients {
        println!("Name: {} Id: {}",c.get_name(),c.get_id());
    } 

    Ok(())
}