//! TMF645 Create Qualification Example

use tmf_client::{common::tmf_error::TMFError, Operations, TMFClient};
use tmflib::tmf645::check_service_qualification::CheckServiceQualification;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf645")]
    {
        use tmf_client::DEFAULT_PORT;

        let qualification = CheckServiceQualification::new("A Qualification Example");

        let new_qual = TMFClient::new("https://localhost:8001", Some(DEFAULT_PORT))
            .tmf645()
            .check_qualifcation()
            .create(qualification)?;

        dbg!(new_qual);
    }
    Ok(())
}
