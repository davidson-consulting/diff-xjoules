use std::collections::HashMap;

use serde_derive::Serialize;

use super::{
    measure::version_measure::VersionMeasure,
    steps::{
        test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
        test_selection::TestSelection,
    },
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
    pub decisions: Vec<DecisionData>,
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
            decisions: Vec::new(),
        };
    }
}

#[derive(Serialize)]
pub enum Decision {
    Pass,
    Break,
    NotApplicable,
}

impl Decision {
    pub fn from_bool(decision: bool) -> Decision {
        if decision {
            return Decision::Pass;
        } else {
            return Decision::Break;
        }
    }
}

#[derive(Serialize)]
pub struct DecisionData {
    pub decision: Decision,
    pub test_selection: TestSelection,
    pub test_filter: String,
    pub mark_strategy: String,
}

impl DecisionData {
    pub fn not_applicable(test_filter: &TestFilterEnum) -> DecisionData {
        return DecisionData {
            decision: Decision::NotApplicable,
            test_selection: TestSelection::new(),
            test_filter: test_filter.to_string(),
            mark_strategy: String::from(""),
        };
    }
    pub fn new(
        decision: Decision,
        test_selection: &TestSelection,
        test_filter: &TestFilterEnum,
        mark_strategy: &MarkStrategyEnum,
    ) -> DecisionData {
        return DecisionData {
            decision,
            test_selection: TestSelection::clone(test_selection),
            test_filter: test_filter.to_string(),
            mark_strategy: mark_strategy.to_string(),
        };
    }
}
