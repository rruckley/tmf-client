//! Get Organization Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf632")]
use tmf_client::{Operations, TMFClient};
#[cfg(feature = "tmf632")]
use tmflib::{HasId,HasName};

fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf632")]
    {
        let organizations = TMFClient::new("https://localhost:8001")
        .tmf632()
        .organization()
        .list(None)?;

        for o in organizations {
            println!("Name: {} , Id: {}",o.get_name(),o.get_id());
        }

    }

    Ok(())
}