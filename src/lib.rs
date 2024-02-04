
pub mod tmf;

use tmf::tmf620::TMF620;
use tmf::tmf622::TMF622;

pub struct TMFClient {
    host : String,
    tmf620 : Option<TMF620>,
    tmf622 : Option<TMF622>,
}

impl TMFClient {
    pub fn new(host : impl Into<String>) -> TMFClient {
        TMFClient {
            host : host.into(),
            tmf620 : None,
            tmf622 : None,
        }
    }

    pub fn tmf620(&mut self) -> TMF620 {
        match self.tmf620.as_mut() {
            Some(tmf) => tmf.clone(),
            None => {
                // Allocate a new instance
                let tmf = TMF620 {};
                self.tmf620 = Some(TMF620 {});
                tmf
            }
        }
    }

    pub fn tmf622(&mut self) -> TMF622 {
        match self.tmf622.as_mut() {
            Some(tmf) => tmf.clone(),
            None => {
                // Allocate a new instance
                let tmf = TMF622 {};
                self.tmf620 = Some(TMF620 {  });
                tmf
            }
        }
    }
}
