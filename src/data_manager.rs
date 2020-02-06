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




pub fn backup(archive_name: String, archive_base_dir: String, archive_path: String) -> Result<(), Error> {
            let file = format!("{}.tar.gz", archive_name);
            let file_handle = File::create(file)?;
            let enc = GzEncoder::new(file_handle, Compression::new(3)); // new allows for various levels of compression.
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all(archive_base_dir, archive_path).unwrap();

        Ok(())
}


pub fn restore(archive_name: String, storage_path: String) -> Result<(), Error> {
    let file = format!("{}/{}.tar.gz", storage_path, archive_name);
    let file_handle = File::open(file)?;
    let tar = GzDecoder::new(file_handle);
    let mut archive = Archive::new(tar);
    archive.unpack(&storage_path)?;

    Ok(())
}