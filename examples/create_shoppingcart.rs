//! Create Shopping Cart Example
//!

use tmf_client::common::tmf_error::TMFError;

fn main() -> Result<(), TMFError> {
    // This example demonstrates how to create a shopping cart using the TMF663 API.
    // Ensure you have the necessary dependencies and the TMFClient set up in your project.

    use tmf_client::{Operations, TMFClient};
    #[cfg(feature = "tmf663")]
    use tmflib::tmf663::shopping_cart::ShoppingCart;

    // Initialize the TMF client with the base URI of your TMF server
    let mut client = TMFClient::new("https://localhost:8001", None);

    let cart = ShoppingCart::new();

    let out = client.tmf663().shopping_cart().create(cart)?;

    dbg!(out);

    Ok(())
}
