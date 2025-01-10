//! Product Offering Example

use tmf_client::{TMFClient,QueryOptions,Operations};
use tmflib::{HasId,HasName};

fn main() {
    let mut client = TMFClient::new("http://localhost:8001");
    let filter = QueryOptions::default()
    //.limit(2)
    .offset(0);
    let tmf = client.tmf620()
    .product_specification()
    .list(Some(filter));
    match tmf {
        Ok(r) => {
            // It worked
            for c in r {
                println!("Entry: {} [{}]",c.get_name(),c.get_id())
            }
        },
        Err(e) => {
            println!("Error: {:?}",e)
        },
    }
}