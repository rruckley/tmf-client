//! TMF639 Resource Inventory API Example

use tmf_client::{BlockingOperations, TMFClient};
use tmf_client::common::tmf_error::TMFError;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf639")]
    {
        use tmflib::HasName;
        use tmf_client::BlockingOperations;

        let resources = TMFClient::new("https://localhost:8001", None)
            .tmf639()
            .resource()
            .list(None)?;

        for resource in resources {
            println!("Name: {}", resource.get_name())
        }
    }

    Ok(())
}
