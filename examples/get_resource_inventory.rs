//! TMF639 Resource Inventory API Example

use tmf_client::common::tmf_error::TMFError;
use tmf_client::TMFClient;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf639")]
    {
        use tmf_client::BlockingOperations;
        use tmflib::HasName;

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
