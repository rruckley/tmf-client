//! Create Customer Example

use tmf_client::{Operations, TMFClient};
use tmf_client::common::tmf_error::TMFError;
use tmflib::tmf632::organization_v4::Organization;
use tmflib::tmf629::customer::Customer;

fn main() -> Result<(),TMFError> {
    let org = Organization::new("An Organization");

    let customer = Customer::new(org);

    let new_customer = TMFClient::new("http://localhost:8001")
        .tmf629()
        .customer()
        .create(customer)?;

    dbg!(new_customer);

    Ok(())
}