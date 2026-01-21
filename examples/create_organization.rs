//! Create Organization Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "blocking")]
use tmf_client::{BlockingOperations, TMFClient};
#[cfg(not(feature = "blocking"))]
use tmf_client::{AsyncOperations, TMFClient};
#[cfg(feature = "tmf632")]
use tmflib::tmf632::organization_v4::Organization;

use tmf_client::DEFAULT_PORT;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "blocking")]
    {
        let org = Organization::new("An Organization");

        let client = TMFClient::new("https://localhost:8001", Some(DEFAULT_PORT))
            .tmf632()
            .organization()
            .create(org)?;

        dbg!(client);
    }

    Ok(())
}
