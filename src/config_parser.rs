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
extern crate toml;


use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use toml::Value;


struct Config {
    debug: Bool,
    backup_type: String,
    storage_location: String,
    backup_path: String
    server: Server,
    clients: Vec<Client>
}

struct Server {
    hostname: String,
    ip: String,
    system_group: String
}

struct Client {
    hostname: String,
    ip: String,
    system_group: String
}


pub fn config(filename: &str) -> Config {
    // config will return a Config struct populated
    // with data from the configuration file.
    let token = ["debug", "server", "clients", "storage", "exclude_dir"]

}

fn read_config(filename: &str) -> toml::value::Value {
    let fh = File::open(filename).expect("unable to open file.");
    let mut buf_reader = BufReader::new(fh);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let settings: Value = toml::from_str(&contents).unwrap();
    settings
}

