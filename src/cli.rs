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
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// pub enum ServiceType {
//     Server {
//         ip_address: String,
//         port: String
//     },
//     Client {
//         ip_address: String,
//         port: String
//     }
// }

#[derive(StructOpt, Debug)]
#[structopt(name = "Systash")]
pub struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    // Backup behavior flag, this will indicate what action to take.
    /// Create a system backup.
    #[structopt(short = "b", long = "backup")]
    pub backup: bool,

    // Restore behavior flag.
    /// Restore a given file or path.
    #[structopt(short = "r", long = "restore")]
    pub restore: bool,

    /// Build system recovery image.
    #[structopt(short ="i", long = "recovery_image")]
    pub build_recovery: bool,

    // Storage path will define the path where the image is written.
    /// Storage path for backup and recovery images.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub storage_path: PathBuf,

    /// Backup path to process.
    #[structopt(name = "Path", parse(from_os_str))]
    pub backup_path: Vec<PathBuf>,

    /// Timestamp for which to recovery from.
    #[structopt(short = "ts", long = "timestamp")]
    pub timestamp: String,

    /// System label or node name to attach to the backup image.
    #[structopt(short = "l", long = "label")]
    pub label: String,

    // System group label will allow user to create categories of
    // machines.
    /// System group label which allow for logical grouping of machines.
    #[structopt(short = "s", long = "system_group")]
    pub system_group: String,

    // Backup type will allow for a full system back up or
    // differential. Default should be differential for
    // time and resource efficiencies.
    /// Backup Type [ Full or Differential ]
    #[structopt(short = "bt", long = "type")]
    pub backup_type: String,

    // // Service Type will be a switch to allow the activation
    // // of a server (API service) or a client (API consumer)
    // /// Service Type selector [ stand-alone, server, client ] if
    // /// none are provided stand-alone is default.
    // #[structopt(subcommand)]
    // pub service_type: ServiceType
}