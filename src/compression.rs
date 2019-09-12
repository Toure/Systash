extern crate zip;

use std::io::{Seek, Write};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};


pub fn create_zip_archive<T: Seek + Write>(buf: &mut T, input_path) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file_from_path(input_path, FileOptions::default())?;
    writer.finish()?;
    ok(())
}

pub fn create_bz2_archive<T: Seek + Write>(buf: &mut T, input_path) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file(input_path,
                    FileOptions::default().compression_method(zip::CompressionMethod::Bzip2))?;
    writer.finish()?;
    Ok(())
}

