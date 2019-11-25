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
extern crate tar;
extern crate flate2;

use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io::Error;
use tar::Archive;


struct DataManager{
    root_name: String,
    hostname: String,
    backup_type: String,
    system_group: String,
    backup_storage_location: String,
    storage_path: String,
}

trait Backup {
    fn new(root_name: String, hostname: String, backup_type: String, system_group: String,
        backup_storage_location: String, storage_path: String) -> DataManager;

    fn backup(&self) -> Result<(), Error>;
}

trait Restore {
    fn new(root_name: String, hostname: String, backup_type: String, system_group: String,
        backup_storage_location: String, storage_path: String ) -> DataManager;

    fn restore(&self) -> Result<(), Error>;

}

trait Recover {
    fn new(&self) -> DataManager;
    fn build_recovery_image(&self) -> Result<(), Error>;
}

impl Backup for DataManager{

    fn new(root_name: String, hostname: String, backup_type: String, system_group: String,
               backup_storage_location: String, storage_path: String) -> DataManager
               {
                   // Entire point for backup struct implementation.
                   DataManager{
                       root_name, hostname, backup_type, system_group,
                               backup_storage_location, storage_path
                   }
    }

    fn backup(&self) -> Result<(), Error> {
                let archive_name = format!("{}.tar.gz", self.hostname);
                let tar_gz = File::create(archive_name)?;
                let enc = GzEncoder::new(tar_gz, Compression::default());
                let mut tar = tar::Builder::new(enc);
                tar.append_dir_all(self.root_name, self.storage_path).unwrap();

            Ok(())
    }
}

impl Restore for DataManager{

    fn new(root_name: String, hostname: String, backup_type: String, system_group: String,
        backup_storage_location: String, storage_path: String ) -> DataManager
        {
            // Entire point for restore struct implementation.
            DataManager{
                root_name, hostname, backup_type, system_group,
                        backup_storage_location, storage_path
            }
    }
    
    fn restore(&self) -> Result<(), Error> {
        let restore_file = format!("{}/{}.tar.gz", self.backup_storage_location, self.hostname);
        let tar_gz = File::open(restore_file)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(self.storage_path)?;

        Ok(())
    }
}

#[allow(dead_code)]
impl Recover for DataManager{
    fn new(&self) -> DataManager {
        unimplemented!()
    }

    fn build_recovery_image(&self) -> Result<(), Error> {
        unimplemented!()
    }
}