use std::collections::HashMap;

use serde_derive::Deserialize;

use utils::yaml_utils::read_yaml;

use self::{
    steps::{
        test_execution, test_instrumentation,
        test_selection::{self, TestSelection},
    },
    utils::coverage::Coverage,
};

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
    pub instrumentation_cmd: String,
    pub execution_cmd: String,
    pub iteration_warmup: i8,
    pub iteration_run: i8,
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
    pub test_selection: Option<TestSelection>,
    pub data_v1: Option<VersionMeasure>,
    pub data_v2: Option<VersionMeasure>,
}

impl DiffXJoulesData {
    pub fn new() -> DiffXJoulesData {
        return DiffXJoulesData {
            coverage_v1: None,
            coverage_v2: None,
            diff: None,
            test_selection: None,
            data_v1: None,
            data_v2: None,
        };
    }
}

#[derive(Deserialize)]
pub struct VersionMeasure {
    pub test_measures: Vec<TestMeasure>,
}

#[derive(Deserialize)]
pub struct TestMeasure {
    pub test_identifier: String,
    pub measures: Vec<Measure>,
}

#[derive(Deserialize)]
pub struct Measure {
    pub data: Vec<Data>,
}

#[derive(Deserialize)]
pub struct Data {
    pub indicator: String,
    pub value: i128,
}

pub fn run(path_to_configuration_yaml_file: String) {
    let configuration = Configuration::new(path_to_configuration_yaml_file);
    let mut diff_xjoules_data = DiffXJoulesData::new();
    //test_selection::run(&configuration, &mut diff_xjoules_data);
    //test_instrumentation::run(&configuration);
    test_execution::run(&configuration,diff_xjoules_data);
}
