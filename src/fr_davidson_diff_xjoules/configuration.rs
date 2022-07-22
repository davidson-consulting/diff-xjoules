use serde_derive::Deserialize;

use super::{
    steps::test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
    utils::yaml_utils::read_yaml,
};

#[derive(Deserialize)]
pub struct Configuration {
    pub path_v1: String,
    pub path_v2: String,
    pub src_folder: String,
    pub path_output_dir: String,
    pub coverage_cmd: String,
    pub instrumentation_cmd: String,
    pub execution_cmd: String,
    pub iteration_warmup: i8,
    pub iteration_run: i8,
    pub time_to_wait_in_millis: u64,
    pub test_filter: TestFilterEnum,
    pub mark_strategy: MarkStrategyEnum,
    pub indicator_to_consider_for_marking: String,
}

impl Configuration {
    pub fn new(path_to_configuration_file: std::path::PathBuf) -> Configuration {
        return read_yaml::<Configuration>(path_to_configuration_file);
    }
}
