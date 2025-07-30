//! Get Shopping Cart Example

//! Create Shopping Cart Example
//! 

use tmflib::HasId;
use tmf_client::common::tmf_error::TMFError;

fn main() -> Result<(),TMFError>{
    // This example demonstrates how to create a shopping cart using the TMF663 API.
    // Ensure you have the necessary dependencies and the TMFClient set up in your project.

    #[cfg(feature = "tmf663")]
    {
        use tmf_client::{Operations,TMFClient};

        // Initialize the TMF client with the base URI of your TMF server
        let mut client = TMFClient::new("https://localhost:8001",None);

        let out = client
            .tmf663()
            .shopping_cart()
            .list(None)?;

        for cart in out {
                println!("Id: {}",cart.get_id());
        } 
    }
    Ok(())
}