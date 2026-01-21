//! Get Catalog Example

#[cfg(feature = "blocking")]
use tmf_client::{BlockingOperations, QueryOptions, TMFClient};
#[cfg(feature = "tmf620")]
use tmflib::HasName;

fn main() {
    #[cfg(feature = "blocking")]
    {
        let mut client = TMFClient::new("https://localhost:8001", None);
        let filter = QueryOptions::default().limit(2).offset(0);
        let tmf = client.tmf620().catalog().list(Some(filter));
        match tmf {
            Ok(r) => {
                // It worked
                for c in r {
                    println!("Entry: {}", c.get_name())
                }
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
    }
}
