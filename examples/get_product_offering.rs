//! Product Offering Example

#[cfg(feature = "tmf620")]
use tmf_client::{TMFClient,QueryOptions,Operations};
#[cfg(feature = "tmf620")]
use tmflib::{HasId,HasName};

fn main() {
    #[cfg(feature = "tmf620")]
    {
        let mut client = TMFClient::new("http://localhost:8001");
        let filter = QueryOptions::default()
        //.limit(2)
        .offset(0);
        let tmf = client.tmf620()
        .product_offering()
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
}