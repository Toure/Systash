/*Configuration Parser will read in the TOML file past in to
set parameters for backups. 
*/

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Configuration {
    filepath: String,
    nodename: String,
    ip: Option<String>
}

mod config {
    fn parse() {}
}
