//! TMF645 Get Qualification Example

use tmf_client::{common::tmf_error::TMFError, tmf::tmf645};
#[cfg(feature = "tmf645")]
use tmf_client::{Operations, TMFClient};
use tmflib::HasDescription;
#[cfg(feature = "tmf645")]
use tmflib::{HasId,HasName};

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