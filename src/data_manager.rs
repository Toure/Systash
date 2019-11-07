extern crate tar;
extern crate flate2;

use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

struct DataManager{
    archive_name: String,
    backup_path: String,
    label: String,
    backup_log: String
}

#[allow(dead_code)]
impl DataManager{
    pub fn new (){
        unimplemented!()
    }

    fn backup(archive_name: String, 
            backup_path: String, 
            backup_logs: String,
            label: String) -> Result<(), std::io::Error> {
                let tar_gz = File::create(archive_name)?;
                let enc = GzEncoder::new(tar_gz, Compression::default());
                let mut tar = tar::Builder::new(enc);
            tar.append_dir_all(label, backup_path).unwrap();
            tar.finish();
            Ok(())
    }

    fn restore() {
        unimplemented!()
    }

    fn build_recovery_image(){
        unimplemented!()
    }
}
