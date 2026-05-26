//! Create Customer Example

use tmf_client::common::tmf_error::TMFError;
#[cfg(feature = "tmf629")]
use tmf_client::TMFClient;

#[cfg(not(feature = "blocking"))]
use tmf_client::AsyncOperations;

#[cfg(feature = "tmf629")]
use tmflib::tmf629::customer::Customer;
#[cfg(feature = "tmf629")]
use tmflib::tmf632::organization_v4::Organization;

use tmf_client::DEFAULT_PORT;

fn main() -> Result<(), TMFError> {
    #[cfg(feature = "tmf629")]
    {
        use tmf_client::BlockingOperations;

        let org = Organization::new("An Organization Example");

        let customer = Customer::new(org);

        let new_customer = TMFClient::new("https://localhost", Some(DEFAULT_PORT))
            .tmf629()
            .customer()
            .create(customer)?;

        dbg!(new_customer);
    }
    Ok(())
}
