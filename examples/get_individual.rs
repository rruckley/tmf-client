//! Get Individual Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf632")]
use tmf_client::{TMFClient,Operations};
#[cfg(feature = "tmf632")]
use tmflib::{HasId,HasName};


fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf632")]
    {
        let mut client = TMFClient::new("https://localhost:8001");

        let individuals = client
            .tmf632()
            .individual()
            .list(None)?;
    
        for i in individuals {
            println!("Name: {} Id: {}, Gender: {}",i.get_name(),i.get_id(),i.gender.unwrap_or("Gender not set".to_string()));
        }
    
    }

    Ok(())
}