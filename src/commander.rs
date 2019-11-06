use std::process::Command;


fn mkisofs(mkisofs_opt: String, outputfile: String, label: String) {
    let mut mkisofs = Command::new("mkisofs");
    mkisofs.arg("-o {}", outputfile)
           .arg("-b isolinux/isolinux.bin")
           .arg("-c isolinux/boot.cat")
           .arg("-no-emul-boot")
           .arg("-boot-load-size 4")
           .arg("-boot-info-table")
           .arg("-R")
           .arg("-J")
           .arg("-volid {}", label);
}

fn mod_grub() {

}

