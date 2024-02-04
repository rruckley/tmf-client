pub struct TMFClient {
    host : String,
}

impl TMFClient {
    pub fn new(host : impl Into<String>) -> TMFClient {
        TMFClient {
            host : host.into(),
        }
    }
}
