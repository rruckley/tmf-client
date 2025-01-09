//! TMF674 Module

use tmflib::Uri;
use crate::{Operations,HasNew};

/// Product Catalogue API
#[derive(Clone,Default,Debug)]
pub struct TMF674 {
    host : Uri,
}

impl HasNew<TMF674> for TMF674 {
    fn new(host : Uri) -> TMF674 {
        TMF674 {
            host
        }
    }
}