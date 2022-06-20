use serde_derive::Deserialize;

use utils::yaml_utils::read_yaml;

use self::{utils::coverage::Coverage, steps::{test_selection::{self, TestSelection}, test_instrumentation}};

pub mod steps;
pub mod utils;

pub const SUFFIX_V1: &str = "_v1";
pub const SUFFIX_V2: &str = "_v2";

#[derive(Deserialize)]
pub struct Configuration {
    pub path_v1: String,
    pub path_v2: String,
    pub src_folder: String,
    pub path_output_dir: String,
    pub coverage_cmd: String,
    pub instrumentation_cmd: String
}

impl Configuration {
    pub fn new(path_to_configuration_file: String) -> Configuration {
        return read_yaml::<Configuration>(path_to_configuration_file);
    }
}

pub struct DiffXJoulesData {
    pub coverage_v1: Option<Coverage>,
    pub coverage_v2: Option<Coverage>,
    pub diff: Option<String>,
    pub test_selection: Option<TestSelection>
}

impl DiffXJoulesData {
    pub fn new() -> DiffXJoulesData {
        return DiffXJoulesData { 
            coverage_v1: None,
            coverage_v2: None,
            diff: None,
            test_selection: None
         };
    }
}

pub fn run(path_to_configuration_yaml_file: String) {
    let configuration = Configuration::new(path_to_configuration_yaml_file);
    let mut diff_xjoules_data = DiffXJoulesData::new();

    test_selection::run(&configuration, &mut diff_xjoules_data);
    test_instrumentation::run(&configuration);
}