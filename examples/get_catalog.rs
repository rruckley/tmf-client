//! Get Catalog Example

use tmf_client::TMFClient;

fn main() {
    let mut client = TMFClient::new("http://localhost:8080");
    let tmf = client.tmf620();

    dbg!(tmf);
}