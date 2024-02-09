//! Get Category Example

use tmf_client::{TMFClient,QueryOptions};
use tmflib::HasName;

fn main() {
    // Get a list of categories from TMF620
    let mut client = TMFClient::new("http://localhost:8000");
    let filter = QueryOptions::default()
        .limit(2)
        .offset(0);
    let tmf = client.tmf620()
        .category()
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