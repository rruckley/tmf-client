//! Get Category Example

#[cfg(feature = "tmf620")]
use tmf_client::{TMFClient,QueryOptions,Operations};
#[cfg(feature = "tmf620")]
use tmflib::HasName;

fn main() {
    #[cfg(feature = "tmf620")]
    {
        // Get a list of categories from TMF620
        let mut client = TMFClient::new("https://localhost:8001",None);
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
}