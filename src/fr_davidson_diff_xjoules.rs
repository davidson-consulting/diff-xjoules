use serde_derive::Deserialize;

use utils::yaml_utils::read_yaml;

use self::{utils::coverage::Coverage, steps::test_selection};

pub mod steps;
pub mod utils;

#[derive(Deserialize)]
pub struct Configuration {
    pub path_v1: String,
    pub path_v2: String,
    pub cmd_coverage: String,
    pub path_output_coverage: String,
    
}

impl Configuration {
    pub fn new(path_to_configuration_file: String) -> Configuration {
        return read_yaml::<Configuration>(path_to_configuration_file);
    }
}

pub struct DiffXJoulesData {
    pub coverage_v1: Option<Coverage>,
    pub coverage_v2: Option<Coverage>
}

impl DiffXJoulesData {
    pub fn new() -> DiffXJoulesData {
        return DiffXJoulesData { 
            coverage_v1: None,
            coverage_v2: None,
         };
    }
}

pub fn run(path_to_configuration_yaml_file: String) {
    let configuration = Configuration::new(path_to_configuration_yaml_file);
    let diff_xjoules_data = DiffXJoulesData::new();
    
    test_selection::run(configuration, diff_xjoules_data);
}