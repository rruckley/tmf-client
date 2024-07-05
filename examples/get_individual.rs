//! Get Individual Example

use tmf_client::{TMFClient,QueryOptions,Operations};
use tmf_client::common::tmf_error::TMFError;
use tmflib::{HasId,HasName};


fn main() -> Result<(),TMFError> {
    let mut client = TMFClient::new("http://localhost:8000");

    let individuals = client
        .tmf632()
        .individual()
        .list(None)?;

    for i in individuals {
        println!("Name: {} Id: {}, Gender: {}",i.get_name(),i.get_id(),i.gender.unwrap_or("Gender not set".to_string()));
    }

    Ok(())
}