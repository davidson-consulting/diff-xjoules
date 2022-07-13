use serde_derive::{Deserialize, Serialize};

use utils::yaml_utils::read_yaml;

use self::{
    steps::{
        test_execution, test_instrumentation,
        test_selection::{self, TestSelection}, test_mark::{test_filter::TestFilterEnum, mark_strategy::MarkStrategyEnum, self}, test_delta,
    },
    utils::{coverage::Coverage, math, json_utils},
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
    pub time_to_wait_in_millis: u64,
    pub test_filter: TestFilterEnum,
    pub mark_strategy: MarkStrategyEnum,
    pub indicator_to_consider_for_marking: String,
}

impl Configuration {
    pub fn new(path_to_configuration_file: String) -> Configuration {
        return read_yaml::<Configuration>(path_to_configuration_file);
    }
}

pub struct DiffXJoulesData {
    pub coverage_v1: Coverage,
    pub coverage_v2: Coverage,
    pub diff: String,
    pub test_selection: TestSelection,
    pub data_v1: VersionMeasure,
    pub data_v2: VersionMeasure,
    pub median_v1: VersionMeasure,
    pub median_v2: VersionMeasure,
    pub delta: VersionMeasure,
    pub mark_test_selection: TestSelection,
    pub decision: bool,
}

impl DiffXJoulesData {
    pub fn new() -> DiffXJoulesData {
        return DiffXJoulesData {
            coverage_v1: Coverage { test_coverages: Vec::new() },
            coverage_v2: Coverage { test_coverages: Vec::new() },
            diff: String::from(""),
            test_selection: TestSelection::new(),
            data_v1: VersionMeasure { test_measures: Vec::new() },
            data_v2: VersionMeasure { test_measures: Vec::new() },
            median_v1: VersionMeasure { test_measures: Vec::new() },
            median_v2: VersionMeasure { test_measures: Vec::new() },
            delta: VersionMeasure { test_measures: Vec::new() },
            mark_test_selection: TestSelection::new(),
            decision: false,
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
    pub fn find_test_measure(&self, test_identifier: &str) -> Option<&TestMeasure> {
        return self.test_measures.iter()
            .find(|test_measure| test_measure.test_identifier == test_identifier);
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestMeasure {
    pub test_identifier: String,
    pub measures: Vec<Vec<Data>>,
}

impl TestMeasure {
    pub fn get_all_indicators(&self) -> Vec<&String> {
        let mut indicators = Vec::<&String>::new();
        self.measures[0].iter().for_each(|data| indicators.push(&data.indicator));
        return indicators;
    }
    pub fn get_median(&self, indicator: &str) -> f64 {
        let mut values: Vec<f64> = self.measures.iter()
        .map(|datas| 
            datas.iter().find(|data| data.indicator == indicator).unwrap().value
        ).collect();
        return math::median(& mut values);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub indicator: String,
    pub value: f64,
}

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

mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::utils::json_utils;

    #[test]
    fn test_version_measure_find_test_measure() {
        let mut version_measure_1: VersionMeasure = VersionMeasure { test_measures: Vec::new() };
        let test_measure_test_1: TestMeasure = TestMeasure { test_identifier: String::from("test1"), measures: Vec::new() };
        let test_measure_test_2: TestMeasure = TestMeasure { test_identifier: String::from("test2"), measures: Vec::new() };

        version_measure_1.test_measures.push(test_measure_test_1);
        version_measure_1.test_measures.push(test_measure_test_2);

        assert_eq!("test1", version_measure_1.find_test_measure("test1").unwrap().test_identifier);
        assert_eq!("test2", version_measure_1.find_test_measure("test2").unwrap().test_identifier);
        match version_measure_1.find_test_measure("does_not_exist") {
            Some(_) => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn test_test_measure_get_median() {
        let mut test_measure_test_1: TestMeasure = TestMeasure { test_identifier: String::from("test1"), measures: Vec::new() };
        let mut data_1 = Vec::new();
        data_1.push(Data {indicator: String::from("instructions"), value: 20.0});
        data_1.push(Data {indicator: String::from("cycles"), value: 2000.0});
        test_measure_test_1.measures.push(data_1);

        let mut data_2 = Vec::new();
        data_2.push(Data {indicator: String::from("instructions"), value: 10.0});
        data_2.push(Data {indicator: String::from("cycles"), value: 1000.0});
        test_measure_test_1.measures.push(data_2);

        let mut data_3 = Vec::new();
        data_3.push(Data {indicator: String::from("instructions"), value: 30.0});
        data_3.push(Data {indicator: String::from("cycles"), value: 3000.0});
        test_measure_test_1.measures.push(data_3);

        assert_eq!(20.0, test_measure_test_1.get_median("instructions"));
        assert_eq!(2000.0, test_measure_test_1.get_median("cycles"));
    }

    #[test]
    fn test_version_measure_merge() {
        let mut version_measure_1: VersionMeasure = VersionMeasure { test_measures: Vec::new() };
        let mut test_measure_test_1: TestMeasure = TestMeasure { test_identifier: String::from("test1"), measures: Vec::new() };
        let mut data_1 = Vec::new();
        data_1.push(Data {indicator: String::from("instructions"), value: 1000.0});
        data_1.push(Data {indicator: String::from("cycles"), value: 2000.0});
        test_measure_test_1.measures.push(data_1);
        version_measure_1.test_measures.push(test_measure_test_1);
        let mut test_measure_test_2: TestMeasure = TestMeasure { test_identifier: String::from("test2"), measures: Vec::new() };
        let mut data_2 = Vec::new();
        data_2.push(Data {indicator: String::from("instructions"), value: 1000.0});
        data_2.push(Data {indicator: String::from("cycles"), value: 2000.0});
        test_measure_test_2.measures.push(data_2);
        version_measure_1.test_measures.push(test_measure_test_2);

        let mut version_measure_2: VersionMeasure = VersionMeasure { test_measures: Vec::new() };
        let mut test_measure_test_3: TestMeasure = TestMeasure { test_identifier: String::from("test1"), measures: Vec::new() };
        let mut data_3 = Vec::new();
        data_3.push(Data {indicator: String::from("instructions"), value: 1000.0});
        data_3.push(Data {indicator: String::from("cycles"), value: 2000.0});
        test_measure_test_3.measures.push(data_3);
        version_measure_2.test_measures.push(test_measure_test_3);
        let mut test_measure_test_4: TestMeasure = TestMeasure { test_identifier: String::from("test3"), measures: Vec::new() };
        let mut data_4 = Vec::new();
        data_4.push(Data {indicator: String::from("instructions"), value: 1000.0});
        data_4.push(Data {indicator: String::from("cycles"), value: 2000.0});
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
