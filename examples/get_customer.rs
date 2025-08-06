//! Get Customer Exfample

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf629")]
use tmf_client::{Operations, TMFClient};
#[cfg(feature = "tmf629")]
use tmflib::{HasId, HasName};

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf629")]
    {
        let clients = TMFClient::new("https://localhost:8001", None)
            .tmf629()
            .customer()
            .list(None)?;

        for c in clients {
            println!("Name: {} Id: {}", c.get_name(), c.get_id());
        }
    }

    Ok(())
}
