use serde_derive::{Deserialize, Serialize};

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
    pub time_to_wait_in_millis: u64
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

#[derive(Serialize, Deserialize)]
pub struct VersionMeasure {
    pub test_measures: Vec<TestMeasure>,
}

impl VersionMeasure {
    pub fn merge(& mut self, that: VersionMeasure) {
        for test_measure in that.test_measures.into_iter() {
            match self.test_measures
                .iter_mut()
                .find(|test_measure_to_update| {
                    test_measure_to_update.test_identifier == test_measure.test_identifier
                }) {
                    Some(test_measure_found) => test_measure_found.measures.extend(test_measure.measures),
                    None => self.test_measures.push(test_measure)
                }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestMeasure {
    pub test_identifier: String,
    pub measures: Vec<Vec<Data>>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub indicator: String,
    pub value: i128,
}

pub fn run(path_to_configuration_yaml_file: String) {
    let configuration = Configuration::new(path_to_configuration_yaml_file);
    let mut diff_xjoules_data = DiffXJoulesData::new();
    test_selection::run(&configuration, &mut diff_xjoules_data);
    test_instrumentation::run(&configuration);
    test_execution::run(&configuration, diff_xjoules_data);
}
 

mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::utils::json_utils;

    #[test]
    fn test_version_measure_merge() {
        let mut version_measure_1: VersionMeasure = VersionMeasure { test_measures: Vec::new() };
        let mut test_measure_test_1: TestMeasure = TestMeasure { test_identifier: String::from("test1"), measures: Vec::new() };
        let mut data_1 = Vec::new();
        data_1.push(Data {indicator: String::from("instructions"), value: 1000});
        data_1.push(Data {indicator: String::from("cycles"), value: 2000});
        test_measure_test_1.measures.push(data_1);
        version_measure_1.test_measures.push(test_measure_test_1);
        let mut test_measure_test_2: TestMeasure = TestMeasure { test_identifier: String::from("test2"), measures: Vec::new() };
        let mut data_2 = Vec::new();
        data_2.push(Data {indicator: String::from("instructions"), value: 1000});
        data_2.push(Data {indicator: String::from("cycles"), value: 2000});
        test_measure_test_2.measures.push(data_2);
        version_measure_1.test_measures.push(test_measure_test_2);

        let mut version_measure_2: VersionMeasure = VersionMeasure { test_measures: Vec::new() };
        let mut test_measure_test_3: TestMeasure = TestMeasure { test_identifier: String::from("test1"), measures: Vec::new() };
        let mut data_3 = Vec::new();
        data_3.push(Data {indicator: String::from("instructions"), value: 1000});
        data_3.push(Data {indicator: String::from("cycles"), value: 2000});
        test_measure_test_3.measures.push(data_3);
        version_measure_2.test_measures.push(test_measure_test_3);
        let mut test_measure_test_4: TestMeasure = TestMeasure { test_identifier: String::from("test3"), measures: Vec::new() };
        let mut data_4 = Vec::new();
        data_4.push(Data {indicator: String::from("instructions"), value: 1000});
        data_4.push(Data {indicator: String::from("cycles"), value: 2000});
        test_measure_test_4.measures.push(data_4);
        version_measure_2.test_measures.push(test_measure_test_4);

        assert_eq!(2, version_measure_1.test_measures.len());
        assert_eq!(1, version_measure_1.test_measures[0].measures.len());
        assert_eq!(1, version_measure_1.test_measures[1].measures.len());
        
        version_measure_1.merge(version_measure_2);

        assert_eq!(3, version_measure_1.test_measures.len());
        assert_eq!(2, version_measure_1.test_measures[0].measures.len());
        assert_eq!(1, version_measure_1.test_measures[1].measures.len());

    }
}
