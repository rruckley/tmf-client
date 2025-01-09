//! TMF633 Module

use tmflib::Uri;

use crate::{Operations,HasNew};

/// Product Catalogue API
#[derive(Clone,Default,Debug)]
pub struct TMF633 {
    host : Uri,
}

impl HasNew<TMF633> for TMF633 {
    fn new(host : Uri) -> TMF633 {
        TMF633 {
            host
        }
    }
}