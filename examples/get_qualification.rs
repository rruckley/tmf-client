//! TMF645 Get Qualification Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf645")]
use tmf_client::{Operations, TMFClient};
#[cfg(feature = "tmf645")]
use tmflib::{HasId,HasDescription};

fn main() -> Result<(),TMFError> {
   #[cfg(feature = "tmf645")] 
   {
        let qualifications = TMFClient::new("http://localhost:8001")
            .tmf645()
            .check_qualifcation()
            .list(None)?;

        for q in qualifications {
            println!("Name: {} Id: {}",q.get_description(),q.get_id());
        }
   }

   Ok(())
}