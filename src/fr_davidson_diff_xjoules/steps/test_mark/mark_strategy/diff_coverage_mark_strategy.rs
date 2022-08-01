use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use super::MarkStrategy;

pub struct DiffCodeCoverageMarkStrategy {}

impl MarkStrategy for DiffCodeCoverageMarkStrategy {
    fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool {
        return test_selection
            .test_selection
            .iter()
            .filter(|selected_test| data.delta.find_test_measure(selected_test).is_some())
            .map(|selected_test| {
                let delta_test = data.delta.find_test_measure(selected_test).unwrap();
                let considered_delta = delta_test.measures[0]
                    .iter()
                    .find(|data| {
                        data.indicator
                            .eq(&configuration.indicator_to_consider_for_marking)
                    })
                    .unwrap();
                return considered_delta.value
                    * (*data
                        .nb_modified_lines_exec_per_test_identifier
                        .get(selected_test)
                        .unwrap() as f64)
                    / (data.nb_total_nb_modified_lines as f64);
            })
            .sum::<f64>()
            < 0.0;
    }
}

impl DiffCodeCoverageMarkStrategy {
    pub fn new() -> DiffCodeCoverageMarkStrategy {
        DiffCodeCoverageMarkStrategy {}
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use crate::fr_davidson_diff_xjoules::{
        measure::version_measure::VersionMeasure,
        steps::test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
        utils::json_utils,
    };

    #[test]
    fn test_decide() {
        let configuration = Configuration {
            path_v1: String::from("diff-jjoules/src/test/resources/diff-jjoules-toy-java-project"),
            path_v2: String::from(
                "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2",
            ),
            src_folder: String::from("src/main/java"),
            path_output_dir: String::from("target"),
            coverage_cmd: String::from(""),
            instrumentation_cmd: String::from(""),
            execution_cmd: String::from(""),
            iteration_warmup: 1,
            iteration_run: 3,
            time_to_wait_in_millis: 500,
            test_filters: vec![TestFilterEnum::All],
            mark_strategies: vec![MarkStrategyEnum::DiffCov],
            indicator_to_consider_for_marking: String::from("UNHALTED_REFERENCE_CYCLES"),
        };
        let mut data = DiffXJoulesData::new();
        data.delta = json_utils::read_json::<VersionMeasure>("test_resources/delta.json");
        let test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        data.nb_total_nb_modified_lines = 12;
        data.nb_modified_lines_exec_per_test_identifier = HashMap::<String, i32>::new();
        data.nb_modified_lines_exec_per_test_identifier
            .insert(String::from("fr.davidson.AppTest#testAddedStatement"), 1);
        data.nb_modified_lines_exec_per_test_identifier
            .insert(String::from("fr.davidson.AppTest#testRemovedStatement"), 1);
        data.nb_modified_lines_exec_per_test_identifier
            .insert(String::from("fr.davidson.AppTest#testUpdatedStatement"), 1);
        data.nb_modified_lines_exec_per_test_identifier.insert(
            String::from("fr.davidson.AppTest#testAddedAndRemovedStatement"),
            2,
        );
        assert!(!configuration
            .mark_strategies[0]
            .decide(&configuration, &data, &test_selection));
    }
}
