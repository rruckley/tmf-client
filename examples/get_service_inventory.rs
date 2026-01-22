//! TMF638 Service Inventory API Example

use tmf_client::common::tmf_error::TMFError;

#[cfg(feature = "blocking")]
use tmf_client::{BlockingOperations, TMFClient};

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "blocking")]
    {
        use tmf_client::QueryOptions;

        let opt = QueryOptions::default()
            // .name("SD L3 Service CFS Instance #0123087452")
            .limit(1)
            .offset(1);

        let services = TMFClient::new("https://localhost:8001", None)
            .tmf638()
            .service()
            // .list(Some(opt))?;
            .list(Some(opt))?;

        let service = services.first().unwrap();

        let bandwidth = service.get_characteristics("bandwidth");

        dbg!(bandwidth);
    }

    Ok(())
}
