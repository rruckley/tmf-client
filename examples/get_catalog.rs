//! Get Catalog Example
use log::info;
#[cfg(feature = "blocking")]
use tmf_client::{BlockingOperations, QueryOptions, TMFClient};
#[cfg(feature = "tmf620")]
use tmflib::HasName;

fn main() {
    #[cfg(feature = "blocking")]
    {
        // print!("Create Client...");
        let mut client = TMFClient::new("https://localhost", None);
        // println!("Done!");
        // print!("Set Options...");
        let filter = QueryOptions::default().limit(2).offset(0);
        // println!("Done!");
        // print!("Get Catalog...");
        let tmf = client.tmf620().catalog().list(Some(filter));
        // println!("Done!");
        match tmf {
            Ok(r) => {
                // It worked
                for c in r {
                    println!("Entry: {}", c.get_name())
                }
            }
            Err(e) => {
                println!("Got an Error: {:?}", e)
            }
        }
    }
}
