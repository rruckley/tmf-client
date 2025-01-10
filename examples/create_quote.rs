//! Create Quote Example
//! 
use tmf_client::{Operations, TMFClient};
use tmf_client::common::tmf_error::TMFError;
use tmflib::tmf648::quote::Quote;


fn main() -> Result<(),TMFError> {
    let quote = Quote::new();

    let new_quote = TMFClient::new("http://localhost:8001")
        .tmf648()
        .quote()
        .create(quote)?;


    dbg!(new_quote);

    Ok(())
}