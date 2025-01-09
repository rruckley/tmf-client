//! Create Organization Example

use tmf_client::{Operations, TMFClient};
use tmf_client::common::tmf_error::TMFError;
use tmflib::tmf632::organization_v4::Organization;

fn main() -> Result<(),TMFError> {
    let org = Organization::new("An Organization");

    let client = TMFClient::new("http://localhost:8001")
        .tmf632()
        .organization()
        .create(org)?;

    dbg!(client);

    Ok(())
}