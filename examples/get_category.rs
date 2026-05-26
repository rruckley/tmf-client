//! Get Category Example

#[cfg(feature = "blocking")]
use tmf_client::{BlockingOperations, QueryOptions, TMFClient};

fn main() {
    #[cfg(feature = "blocking")]
    {
        use tmflib::HasName;
        // Get a list of categories from TMF620
        let mut client = TMFClient::new("https://localhost", None);
        let filter = QueryOptions::default().limit(2).offset(0);
        let tmf = client.tmf620().category().list(Some(filter));
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
