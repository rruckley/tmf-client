//! Create Individual Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf632")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "tmf632")]
use tmf_client::{Operations, TMFClient};


fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf632")]
    {
        let individual = Individual::new("John Example Citizen")
            .email("John.q.cititzen@example.com")
            .gender("Male")
            .title("Master");

        let mut client = TMFClient::new("https://localhost:8001");

        let new_individual = client.tmf632().individual().create(individual)?;

        dbg!(new_individual);

    }

    Ok(())
}