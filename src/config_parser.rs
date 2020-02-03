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
extern crate serde_derive;
extern crate serde;

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

#[derive(Deserialize)]
struct Config {
    logging: HashMap<String, String>,
    backup_path: Backup,
    storage: Storage
}

#[derive(Deserialize)]
struct Backup {
    base_dir: String,
    exclude_dir: Vec<String>,
    system_group: String
}

#[derive(Deserialize)]
struct Storage {
    archives: String,
    host: String,
    backend: String,
    ip: String,
    remote_mount_point: String
}

fn main() {
    let file = "config/systash.toml";
    let output = read_config(&file);
    let to_find = ["logging", "backup_path", "storage"];
    for member in &to_find {
        match output.member {
            Some(element) => println!("{}", element),
            None => println!("{} can not be found", member)
        }
    }
}

fn read_config(filename: &str) -> Config {
    let file = match File::open(&filename) {
        Err(why) => panic!("couldn't open {}: {}", &filename,
                                                   why.description()),
        Ok(file) => file,
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let package_info: Config = toml::from_str(&contents).unwrap();
    package_info
}