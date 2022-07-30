use std::collections::HashMap;

use super::{
    measure::version_measure::VersionMeasure, steps::test_selection::TestSelection,
    utils::coverage::Coverage,
};

pub struct DiffXJoulesData {
    pub coverage_v1: Coverage,
    pub coverage_v2: Coverage,
    pub diff: String,
    pub nb_total_nb_modified_lines: i32,
    pub test_selection: TestSelection,
    pub nb_modified_lines_exec_per_test_identifier: HashMap<String, i32>,
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
            coverage_v1: Coverage {
                test_coverages: Vec::new(),
            },
            coverage_v2: Coverage {
                test_coverages: Vec::new(),
            },
            diff: String::from(""),
            nb_total_nb_modified_lines: 0,
            test_selection: TestSelection::new(),
            nb_modified_lines_exec_per_test_identifier: HashMap::<String, i32>::new(),
            data_v1: VersionMeasure {
                test_measures: Vec::new(),
            },
            data_v2: VersionMeasure {
                test_measures: Vec::new(),
            },
            median_v1: VersionMeasure {
                test_measures: Vec::new(),
            },
            median_v2: VersionMeasure {
                test_measures: Vec::new(),
            },
            delta: VersionMeasure {
                test_measures: Vec::new(),
            },
            mark_test_selection: TestSelection::new(),
            decision: false,
        };
    }
}
