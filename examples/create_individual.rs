//! Create Individual Example

use tmflib::tmf632::individual_v4::Individual;
use tmf_client::{Operations, TMFClient};
use tmf_client::common::tmf_error::TMFError;

fn main() -> Result<(),TMFError> {
    let individual = Individual::new("John Quinten Citizen");

    let mut client = TMFClient::new("http://localhost:8000");

    let new_individual = client.tmf632().individual().create(individual)?;

    dbg!(new_individual);

    Ok(())
}