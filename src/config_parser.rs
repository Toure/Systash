// Config parser will allow for default values to be set from a configuration file,
// such as inventory list, backup schedule, and other features as they are developed.
extern crate toml;
extern crate serde_derive;

use toml::{Value, de::Error};
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    inventory: Inventory
}

#[derive(Deserialize)]
struct Inventory {
    hostname: Vec<String>,
    backup_path: String,
    backup_storage: String

}

fn toml_file_parser(config_file: String){
    // parse the given toml config file and return
    // an content object.
    unimplemented!()
}


pub fn config_parse(toml_content: String) -> Result<(), Error> {
    // Config parser will read in default path to the
    // config file and populate the internal variables unless
    // overridden from the command line.
    let package_info: Config = toml::from_str(toml_content)?;
    Ok(())


}