//! Create Organization Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf632")]
use tmf_client::{Operations, TMFClient};
#[cfg(feature = "tmf632")]
use tmflib::tmf632::organization_v4::Organization;

use tmf_client::DEFAULT_PORT;

fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf632")]
    {
        let org = Organization::new("An Organization");

        let client = TMFClient::new("https://localhost:8001",Some(DEFAULT_PORT))
            .tmf632()
            .organization()
            .create(org)?;
    
        dbg!(client);    
    }

    Ok(())
}