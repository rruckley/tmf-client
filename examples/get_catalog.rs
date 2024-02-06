//! Get Catalog Example

use tmf_client::{TMFClient,QueryOptions};

fn main() {
    let mut client = TMFClient::new("http://localhost:8080");
    let filter = QueryOptions::default()
        .limit(5)
        .offset(0);
    let tmf = client.tmf620()
        .catalog()
        .list(Some(filter));
    match tmf {
        Ok(r) => {
            // It worked
            println!("It worked: {} items",r.len())
        },
        Err(e) => {
            println!("Error: {:?}",e)
        },
    }
}