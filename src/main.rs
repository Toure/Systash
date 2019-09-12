use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "Graphene")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
    
    // Compresion type selection the default will be tar archive with
    // gzip compression.
    /// archive type (zip, tgz = "tar gzip")
    #[structopt(short = "a", long, default_value = "tgz")]
    archive: u8,

    // Bootstrap flag, is used to activate the creation of an iso image.
    /// Create bootable iso image.
    #[structopt(short = "b", long)]
    bootstrap: bool,

    /// Output file
    #[structopt(short = "o", long, parse(from_os_str))]
    output: PathBuf,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
    
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
