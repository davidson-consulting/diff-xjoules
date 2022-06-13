use serde_derive::Deserialize;

use utils::files_utils::read_yaml;

pub mod utils;

#[derive(Deserialize)]
pub struct Configuration {
    pub path_v1: String,
    pub path_v2: String,
    pub cmd_coverage: String,
}

impl Configuration {
    pub fn new(path_to_configuration_file: String) -> Configuration {
        return read_yaml::<Configuration>(path_to_configuration_file);
    }
}