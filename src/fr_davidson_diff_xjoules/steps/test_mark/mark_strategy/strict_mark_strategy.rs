use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use super::MarkStrategy;

pub struct StrictMarkStrategy {}

// For now, we base all our MarkStrategy on cycles, but we could give as input which indicator is to be taken into account
impl MarkStrategy for StrictMarkStrategy {
    fn decide(
        self,
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

impl StrictMarkStrategy {
    pub fn new() -> StrictMarkStrategy {
        StrictMarkStrategy {}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fr_davidson_diff_xjoules::{
        measure::version_measure::VersionMeasure,
        steps::test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
        utils::json_utils,
    };

    #[test]
    fn test_decide() {
        let mark_strategy = StrictMarkStrategy::new();
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
            test_filter: TestFilterEnum::ALL,
            mark_strategy: MarkStrategyEnum::STRICT,
            indicator_to_consider_for_marking: String::from("UNHALTED_REFERENCE_CYCLES"),
        };
        let mut data = DiffXJoulesData::new();
        data.delta = json_utils::read_json::<VersionMeasure>("test_resources/delta.json");
        let test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        assert!(!mark_strategy.decide(&configuration, &data, &test_selection));
    }
}
