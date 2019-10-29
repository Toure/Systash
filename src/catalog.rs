// Catalog module will contain logic to record backup information such as files written,
// and other attributes.

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};
use time::Timespec;

#[derive(Debug)]
struct Catolog {
    id: i32,
    file: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}
impl Catolog {
    // add the option later to determine the type
    // of connection, ie: in memory or on disk.
    pub fn new() -> Catolog{
        Connection::open_in_memory()?;
    }

    pub fn init_database() {
    conn.execute(
        "CREATE TABLE catalog (
                  id              INTEGER PRIMARY KEY,
                  file            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  )",
        NO_PARAMS,
    )?;
    }   

    pub fn check_database(conn) -> Result<()> {
        // check database status for creation

    }

    pub fn write_info(conn ) -> Result<()> {

        conn.execute(
            "INSERT INTO catalog (file, time_created, data)
                    VALUES (?1, ?2, ?3)",
            &[&me.file as &ToSql, &me.time_created, &me.data],
        )?;
        Ok(())
    }

    pub fn read_info(conn) {
        
    }
}


