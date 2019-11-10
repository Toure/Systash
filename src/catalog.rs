// Catalog module will contain logic to record backup information such as files written,
// and other attributes.
use std::time::{Duration, Instant};

pub struct Catalog {
    label: i32,
    file: String,
    time_created: String,
}

#[allow(dead_code)]
impl Catalog {
    // 
    
    pub fn new() -> Catalog{
        unimplemented!()
    }
    
    pub fn gen_catalog(label: Sting, filename: String, time_created: String){
        // Generate catalog will return a document for the system
        // the corresponds to the backup.
        unimplemented!()
    }
}


