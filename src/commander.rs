// Commander module will serve as a place to interface with shell utilities, until
// they are converted to native rust.

use std::process::Command;

#[allow(dead_code)]
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
fn mod_grub() {
    unimplemented!()
}
