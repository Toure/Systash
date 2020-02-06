// Copyright 2019 Toure Dunnon <tdunnon@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SQLite interface library.

extern crate rusqlite;

use rusqlite::{Connection, Result, NO_PARAMS};


pub fn connectdb(db_filename: String) -> Result<(Connection)> {
    let conn = Connection::open(db_filename)?;
    Ok(conn)
}

pub fn initdb(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(" create table if not exists backups(
        id integer primary key,
        ")
}

fn updatedb(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("")
    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("")
    tx.commit()
}
