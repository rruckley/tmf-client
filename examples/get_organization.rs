//! Get Organization Example

use tmf_client::common::tmf_error::TMFError;
use tmf_client::{Operations, TMFClient};
use tmflib::{HasId,HasName};

fn main() -> Result<(),TMFError> {

    let organizations = TMFClient::new("http://localhost:8001")
        .tmf632()
        .organization()
        .list(None)?;

    for o in organizations {
        println!("Name: {} , Id: {}",o.get_name(),o.get_id());
    }

    Ok(())
}