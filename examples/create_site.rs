//! Create GeograhpicSite Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf674")]
use tmf_client::{BlockingOperations, TMFClient};
#[cfg(feature = "tmf674")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;
use tmflib::HasDescription;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf674")]
    {
        let site = GeographicSite::new("Example Bad Site")
            .description("Just a sample data payload for testing tmf-client library");

        let new_site = TMFClient::new("https://localhost", None)
            .tmf674()
            .site()
            .create(site)?;

        dbg!(new_site);
    }

    Ok(())
}
