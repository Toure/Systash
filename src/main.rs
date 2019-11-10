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
use structopt::StructOpt;

mod cli; //

mod data_manager; // compression library
mod commander; // system level library
mod db; // database helper functions.


fn main() {
    let opt = cli::Opt::from_args();
    // TODO: base logic to drive functionality.

    // println!("{:#?}", opt);
    //println!("debug level -> {}", opt.debug)
    println!("file path that will be used. {:#?}", opt.backup_path);

}


#[allow(dead_code)]
fn create_backup(backup_type: String, label: String, output: String, 
                 backup_path: String){
    unimplemented!()
}

#[allow(dead_code)]
fn build_recovery_image(label: String, time_stamp: String, 
                        boot_loader: String, backup_path: String) {
    unimplemented!()
}

#[allow(dead_code)]
fn restore_system(label: String, backup_type: String, time_stamp: String,
                  backup_path: String) {
    unimplemented!()
} 
