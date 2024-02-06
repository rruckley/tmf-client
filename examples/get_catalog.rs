//! Get Catalog Example

use tmf_client::{TMFClient,QueryOptions};
use tmflib::HasName;

fn main() {
    let mut client = TMFClient::new("http://localhost:8000");
    let filter = QueryOptions::default()
        .limit(1)
        .offset(1);
    let tmf = client.tmf620()
        .catalog()
        .list(Some(filter));
    match tmf {
        Ok(r) => {
            // It worked
            for c in r {
                println!("Entry: {}",c.get_name())
            }
        },
        Err(e) => {
            println!("Error: {:?}",e)
        },
    }
}