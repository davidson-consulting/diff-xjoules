use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_xjoules_data::DiffXJoulesData,
    steps::test_selection::TestSelection,
};

use super::MarkStrategy;

pub struct AggregateMarkStrategy {}

impl MarkStrategy for AggregateMarkStrategy {
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
                return delta_test.measures[0]
                    .iter()
                    .find(|data| {
                        data.indicator
                            .eq(&configuration.indicator_to_consider_for_marking)
                    })
                    .unwrap()
                    .value;
            })
            .sum::<f64>()
            < 0.0;
    }
}

impl AggregateMarkStrategy {
    pub fn new() -> AggregateMarkStrategy {
        AggregateMarkStrategy {}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fr_davidson_diff_xjoules::{
        measure::{data::Data, test_measure::TestMeasure, version_measure::VersionMeasure},
        steps::test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
        utils::json_utils,
    };

    #[test]
    fn test_decide_pass() {
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
            mark_strategies: vec![MarkStrategyEnum::Aggregate],
            indicator_to_consider_for_marking: String::from("instructions"),
        };
        let mut data = DiffXJoulesData::new();
        let mut delta = VersionMeasure::new();
        let mut test_measure = TestMeasure {
            test_identifier: String::from("test1"),
            measures: Vec::new(),
        };
        let mut data_1 = Vec::new();
        data_1.push(Data::new("instructions", -1000.0));
        test_measure.measures.push(data_1);
        delta.test_measures.push(test_measure);
        data.delta = delta;
        let mut test_selection = TestSelection::new();
        test_selection.test_selection.insert(String::from("test1"));
        assert!(configuration.mark_strategies[0].decide(&configuration, &data, &test_selection));
    }

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
            mark_strategies: vec![MarkStrategyEnum::Aggregate],
            indicator_to_consider_for_marking: String::from("UNHALTED_REFERENCE_CYCLES"),
        };
        let mut data = DiffXJoulesData::new();
        data.delta = json_utils::read_json::<VersionMeasure>("test_resources/delta.json");
        let test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        assert!(!configuration.mark_strategies[0].decide(&configuration, &data, &test_selection));
    }
}
