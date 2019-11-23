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
// MongoDB connection and collection helper functions.

// use mongodb::{Client, ThreadedClient, db::ThreadedDatabase};


// struct DataBase {
//     collection_name: String,
//     document: String,
//     group_name: String,

// }

// impl DataBase {

//     fn new(&self) {
//         Client::connect("127.0.0.1", 27107).expect("Could not connect to database.");
//     }

//     pub fn create_db(&self) {
//         let db_conn = db_connect();
//         let db = db_conn.client("system_backups");
//         let collection = db.collection("system_catalogs");
//     }

//     pub fn insert_db(collection_name: String, document: String) {
//         // insert new documents into the database.
//         // example: insert_one(doc!{ "title": "Back to the Future" }, None).unwrap();
//         unimplemented!()
//     }

//     pub fn update_db(collection_name: String, document: String) -> Result<()> {
//         // updatedb takes a collection name and document to be inserted
//         // into the database.
//         // document example: doc!{ "title" => "Ferris Bueller’s Day Off" };
//         unimplemented!()
//     }

//     pub fn delete_db_collection() {
//         // delete a specified database collection.
//         unimplemented!()
//     }

//     pub fn query_db_collection() {
//         // query a given document.
//         unimplemented!()
//     }
// }