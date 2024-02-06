
#[warn(missing_docs)]

pub mod tmf;
pub mod common;

use tmf::tmf620::TMF620;
use tmf::tmf622::TMF622;

/// Fields for filtering output
#[derive(Clone, Default, Debug)]
pub struct QueryOptions {
    /// Specific set of fields delimited by comma
    fields : Option<String>,
    limit : Option<u16>,
    offset : Option<u16>,
    /// Filter on name
    name : Option<String>,
}

impl QueryOptions {
    pub fn fields(mut self, fields : String) -> QueryOptions {
        self.fields = Some(fields);
        self
    }
    pub fn limit(mut self, limit : u16) -> QueryOptions {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset : u16) -> QueryOptions {
        self.offset = Some(offset);
        self
    }

    pub fn name(mut self, name : impl Into<String>) -> QueryOptions {
        self.name = Some(name.into());
        self
    }
}

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
                let tmf = TMF620::new(self.host.clone());
                self.tmf620 = Some(tmf.clone());
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
                self.tmf622 = Some(TMF622 {  });
                tmf
            }
        }
    }
}
