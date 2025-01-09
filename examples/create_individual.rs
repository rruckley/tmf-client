//! Create Individual Example

use tmflib::tmf632::individual_v4::Individual;
use tmf_client::{Operations, TMFClient};
use tmf_client::common::tmf_error::TMFError;

fn main() -> Result<(),TMFError> {
    let individual = Individual::new("John Quarry Citizen")
        .email("John.q.cititzen@example.com")
        .gender("Male")
        .title("Master");

    let mut client = TMFClient::new("http://localhost:8001");

    let new_individual = client.tmf632().individual().create(individual)?;

    dbg!(new_individual);

    Ok(())
}