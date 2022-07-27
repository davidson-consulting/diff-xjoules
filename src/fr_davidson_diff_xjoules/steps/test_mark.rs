use crate::fr_davidson_diff_xjoules::{Configuration, DiffXJoulesData};

use self::mark_strategy::MarkStrategy;

pub mod mark_strategy;
pub mod test_filter;

pub fn run(configuration: &Configuration, diff_xjoules_data: &mut DiffXJoulesData) {
    let test_selection = configuration
        .test_filter
        .filter(&configuration, diff_xjoules_data);
    if test_selection.test_selection.is_empty() {}
    let mark_strategy = configuration.mark_strategy.get();
    let decision = mark_strategy.decide(configuration, diff_xjoules_data, &test_selection);
    diff_xjoules_data.mark_test_selection = test_selection;
    diff_xjoules_data.decision = decision;
}

#[cfg(test)]
mod test {
    use crate::fr_davidson_diff_xjoules::{
        configuration::Configuration, diff_data::DiffXJoulesData,
        measure::version_measure::VersionMeasure, steps::test_selection::TestSelection,
        utils::json_utils,
    };

    use super::{mark_strategy::MarkStrategyEnum, run, test_filter::TestFilterEnum};

    #[test]
    fn test_run() {
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
            test_filter: TestFilterEnum::All,
            mark_strategy: MarkStrategyEnum::STRICT,
            indicator_to_consider_for_marking: String::from("UNHALTED_REFERENCE_CYCLES"),
        };
        let mut data = DiffXJoulesData::new();
        data.test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        data.delta = json_utils::read_json::<VersionMeasure>("test_resources/delta.json");
        run(&configuration, &mut data);
        assert_eq!(4, data.mark_test_selection.test_selection.len());
        assert!(data
            .mark_test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testAddedStatement"));
        assert!(data
            .mark_test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testAddedAndRemovedStatement"));
        assert!(data
            .mark_test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testUpdatedStatement"));
        assert!(data
            .mark_test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testRemovedStatement"));
        assert!(!data.decision);
    }
}
