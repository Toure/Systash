use std::path::PathBuf;
use structopt::StructOpt;   
  
    
#[derive(StructOpt, Debug)]
#[structopt(name = "graphene")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    // Backup behaviour flag, this will indicate what action to take.
    /// Create a system backup.
    #[structopt(short = "b", long = "backup")]
    backup: bool,

    // Restore behaviour flag.
    /// Restore a given file or path.
    #[structopt(short = "r", long = "restore")]
    restore: bool,

    /// Build system recovery image.
    #[structopt(short ="i", long = "recovery_image")]
    build_recovery: bool,

    /// Storage path for backup and recovery images.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// Backup path to process.
    #[structopt(name = "Path", parse(from_os_str))]
    path: Vec<PathBuf>,

    /// Timestamp for which to recovery from.
    #[structopt(short = "t", long = "timestamp")]
    timestamp: String,

    /// System label or node name to attach to the backup image.
    #[structopt(short = "l", long = "label")]
    label: String,

    /// Boot loader type [ GRUB or GRUB2 ]
    #[structopt(short = "B", long = "bootloader")]
    bootloader: String,

    // Backup type will allow for a full system back up or
    // differential. Default should be differential for 
    // time and resource efficentcies.
    /// Backup Type [ Full or Differential ]
    #[structopt(short = "T", long = "type")]
    backuptype: String,
    
}
        
    

fn main() {
    let opt = Opt::from_args();
    // println!("{:#?}", opt);
    //println!("debug level -> {}", opt.debug)
    println!("file path that will be used. {:#?}", opt.path);

}