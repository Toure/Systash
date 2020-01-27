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
// Commander module will serve as a place to interface with shell utilities, until
// they are converted to native rust.
// they are converted to native rust.

use std::process::Command;


#[allow(unused_variables)]
fn mkisofs(mkisofs_opt: String, outputfile: &str, label: &str) {
    let mut mkisofs = Command::new("mkisofs");
    mkisofs.args(&["-o {}", outputfile])
           .args(&["-b", "isolinux/isolinux.bin"])
           .args(&["-c", "isolinux/boot.cat"])
           .arg("-no-emul-boot")
           .args(&["-boot-load-size", "4"])
           .arg("-boot-info-table")
           .arg("-R")
           .arg("-J")
           .args(&["-volid", label])
           .output()
           .expect("Failed to execute mkisofo");
}

#[allow(dead_code)]
pub fn ansible_exec() {
    // Ansible exec will serve as a interface to
    // the command line utiliy to carry out predefined roles.
    unimplemented!()
}