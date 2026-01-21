//! TMF645 Create Qualification Example

use tmf_client::common::tmf_error::TMFError;

#[cfg(feature = "blocking")]
use tmf_client::BlockingOperations;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "blocking")]
    {
        use tmf_client::DEFAULT_PORT;
        use tmf_client::{common::tmf_error::TMFError, TMFClient};
        use tmflib::tmf645::check_service_qualification::CheckServiceQualification;


        let qualification = CheckServiceQualification::new("A Qualification Example");

        let new_qual = TMFClient::new("https://localhost:8001", Some(DEFAULT_PORT))
            .tmf645()
            .check_qualifcation()
            .create(qualification)?;

        dbg!(new_qual);
    }
    Ok(())
}
