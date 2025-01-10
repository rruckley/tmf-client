//! Create GeograhpicSite Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf674")]
use tmf_client::{Operations, TMFClient};
#[cfg(feature = "tmf674")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;


fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf674")]
    {
        let site = GeographicSite::new("Example Site");

        let new_site = TMFClient::new("http://localhost:8001")
            .tmf674()
            .site()
            .create(site)?;
    
    
        dbg!(new_site);
    
    }

    Ok(())
}