use self::{diff_data::DiffXJoulesData, configuration::Configuration, steps::{test_selection, test_instrumentation, test_execution, test_delta, test_mark}};

pub mod steps;
pub mod utils;
pub mod configuration;
pub mod diff_data;
pub mod measure;

pub const SUFFIX_V1: &str = "_v1";
pub const SUFFIX_V2: &str = "_v2";

pub fn run(path_to_configuration_yaml_file: String) {
    let configuration = Configuration::new(path_to_configuration_yaml_file);
    let mut diff_xjoules_data = DiffXJoulesData::new();
    test_selection::run(&configuration, &mut diff_xjoules_data);
    test_instrumentation::run(&configuration);
    test_execution::run(&configuration, &mut diff_xjoules_data);
    test_delta::run(&configuration, &mut diff_xjoules_data);
    test_mark::run(&configuration, &mut diff_xjoules_data);
    if diff_xjoules_data.decision {
        println!("PASS");
    } else {
        println!("BREAK");
    }
}