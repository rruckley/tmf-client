//! TMF637 Product Inventory Management API

use tmf_client::{Operations,TMFClient};
use tmf_client::common::tmf_error::TMFError;

#[cfg(feature = "tmf637")]


fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf637")]
    {
        use tmflib::HasName;

        let products = TMFClient::new("https://localhost:8001",None)
            .tmf637()
            .product()
            .list(None)?;

        for product in products {
            println!("Name: {}",product.get_name())
        }
    }

    Ok(())
}