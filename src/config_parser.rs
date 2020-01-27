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
// Config parser will allow for default values to be set from a configuration file,
// such as inventory list, backup schedule, and other features as they are developed.
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    hostname: String,
    backup_type: String,
    system_group: String,
    storage_location: String,
    backup_path: String
}

trait Parser {
    fn new(&self);
    fn parser(&self);
}

impl Parser for Config{
    fn new(&self){
        // TODO: create struct loader.
        unimplemented!()
    }

    fn parser(&self) -> Result<BTreeMap, Err> {
        // Create a path to the desired file
        let path = Path::new("foo.yml");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                                                    why.description()),
            Ok(file) => file,
        };
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        let content: BTreeMap<String, String> = match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                    why.description()),
            Ok(_) => serde_yaml::from_str(&s).unwrap()
        };
    }
}