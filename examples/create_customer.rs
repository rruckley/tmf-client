//! Create Customer Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf629")]
use tmf_client::{Operations, TMFClient};
#[cfg(feature = "tmf629")]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(feature = "tmf629")]
use tmflib::tmf629::customer::Customer;

fn main() -> Result<(),TMFError> {
    #[cfg(feature = "tmf629")]
    {
        let org = Organization::new("An Organization Example");

        let customer = Customer::new(org);
    
        let new_customer = TMFClient::new("https://localhost:8001")
            .tmf629()
            .customer()
            .create(customer)?;
    
        dbg!(new_customer);
    }
    Ok(())
}