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

fn main() -> Result<()> {
    
    
   
}

fn init_database() {
    conn.execute(
        "CREATE TABLE catalog (
                  id              INTEGER PRIMARY KEY,
                  file            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        NO_PARAMS,
    )?;
}

fn write_info() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    
     conn.execute(
        "INSERT INTO catalog (file, time_created, data)
                  VALUES (?1, ?2, ?3)",
        &[&me.file as &ToSql, &me.time_created, &me.data],
    )?;
    Ok(())
}

fn read_info() {
    
}