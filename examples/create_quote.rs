//! Create Quote Example
//!

use tmf_client::common::tmf_error::TMFError;



fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf632")]
    {
        #[cfg(feature = "tmf648")]
        use tmflib::tmf648::quote::Quote;
        
        use tmf_client::{BlockingOperations, TMFClient};
        let quote = Quote::new();

        let new_quote = TMFClient::new("https://localhost:8001", None)
            .tmf648()
            .quote()
            .create(quote)?;

        dbg!(new_quote);
    }

    Ok(())
}
