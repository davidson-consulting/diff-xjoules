use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use super::MarkStrategy;

pub struct CodeCoverageMarkStrategy {}

impl MarkStrategy for CodeCoverageMarkStrategy {
    fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool {
        for selected_test in test_selection.test_selection.iter() {
            let delta_test = data.delta.find_test_measure(selected_test).unwrap();
            let considered_delta = delta_test.measures[0]
                .iter()
                .find(|data| {
                    data.indicator
                        .eq(&configuration.indicator_to_consider_for_marking)
                })
                .unwrap();
            if considered_delta.value > 0.0 {
                return false;
            }
        }
        return true;
    }
}

impl CodeCoverageMarkStrategy {
    pub fn new() -> CodeCoverageMarkStrategy {
        CodeCoverageMarkStrategy {}
    }
}
