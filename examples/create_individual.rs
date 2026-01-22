//! Create Individual Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(not(feature = "blocking"))]
use tmf_client::{AsyncOperations, TMFClient};
#[cfg(feature = "blocking")]
use tmf_client::{BlockingOperations, TMFClient};
#[cfg(feature = "tmf632")]
use tmflib::tmf632::individual_v4::Individual;

use tmf_client::DEFAULT_PORT;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf632")]
    {
        let individual = Individual::new("John Example Citizen")
            .email("John.q.cititzen@example.com")
            .gender("Male")
            .title("Master");

        let mut client = TMFClient::new("https://localhost:8001", Some(DEFAULT_PORT));

        let new_individual = client.tmf632().individual().create(individual)?;

        dbg!(new_individual);
    }

    Ok(())
}
