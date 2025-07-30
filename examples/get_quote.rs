use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf648")]
use tmf_client::{Operations, TMFClient};


fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf648")]
    {
        let mut client = TMFClient::new("https://localhost:8001");

        let quotes = client
            .tmf648()
            .quote()
            .list(None)?;
    
        for i in quotes {
            use tmflib::{HasDescription, HasName};

            println!("Name: {}, \tDesc: {}",&i.get_name(),i.get_description());
        }
    
    }

    Ok(())
}