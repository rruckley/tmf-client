//! TMF638 Service Inventory API Example

use tmf_client::{Operations,TMFClient};
use tmf_client::common::tmf_error::TMFError;


fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf638")]
    {
        use tmflib::HasName;

        let services = TMFClient::new("https://localhost:8001")
            .tmf638()
            .service()
            .list(None)?;

        for service in services {
            println!("Name: {}",service.get_name())
        }
    }

    Ok(())
}