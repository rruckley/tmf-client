//! Create Customer Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf629")]
use tmf_client::{Operations, TMFClient};
#[cfg(feature = "tmf629")]
use tmflib::tmf629::customer::Customer;
#[cfg(feature = "tmf629")]
use tmflib::tmf632::organization_v4::Organization;

use tmf_client::DEFAULT_PORT;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf629")]
    {
        let org = Organization::new("An Organization Example");

        let customer = Customer::new(org);

        let new_customer = TMFClient::new("https://localhost:8001", Some(DEFAULT_PORT))
            .tmf629()
            .customer()
            .create(customer)?;

        dbg!(new_customer);
    }
    Ok(())
}
