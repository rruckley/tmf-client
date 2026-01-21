//! Get Individual Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "blocking")]
use tmf_client::{BlockingOperations, TMFClient};
#[cfg(feature = "tmf632")]
use tmflib::{HasId, HasName};

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "blocking")]
    {
        let mut client = TMFClient::new("https://localhost:8001", None);

        let individuals = client.tmf632().individual().list(None)?;

        for i in individuals {
            println!(
                "Name: {} Id: {}, Gender: {}",
                i.get_name(),
                i.get_id(),
                i.gender.unwrap_or("Gender not set".to_string())
            );
        }
    }

    Ok(())
}
