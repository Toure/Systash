// Catalog module will contain logic to record backup information such as files written,
// and other attributes.

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};
use time::Timespec;

#[derive(Debug)]
struct Catalog {
    id: i32,
    file: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Catalog {
    // add the option later to determine the type
    // of connection, ie: in memory or on disk.
    
    pub fn new() -> Catalog{
        unimplemented!()
    }

    pub fn init_database() {
        unimplemented!()
    }   

    pub fn check_database(){
        // check database status for creation
        unimplemented!()

    }

    pub fn write_info() -> Result<()> {
        unimplemented!()
    }

    pub fn read_info() {
        unimplemented!()
    }
}


