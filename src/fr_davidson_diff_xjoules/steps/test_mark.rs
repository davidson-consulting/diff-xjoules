use crate::fr_davidson_diff_xjoules::{
    diff_data::{Decision, DecisionData},
    utils::json_utils::{self, JSON_EXTENSION},
    Configuration, DiffXJoulesData,
};

pub mod mark_strategy;
pub mod test_filter;

const DECISION_DATA_FILENAME: &str = "decisions";

pub fn run(configuration: &Configuration, diff_xjoules_data: &mut DiffXJoulesData) {
    for test_filter in configuration.test_filters.iter() {
        let test_selection = test_filter.filter(&configuration, diff_xjoules_data);
        if !test_selection.test_selection.is_empty() {
            for mark_strategy in configuration.mark_strategies.iter() {
                let decision =
                    mark_strategy.decide(configuration, diff_xjoules_data, &test_selection);
                diff_xjoules_data.decisions.push(DecisionData::new(
                    Decision::from_bool(decision),
                    &test_selection,
                    test_filter,
                    mark_strategy,
                ));
            }
        } else {
            diff_xjoules_data
                .decisions
                .push(DecisionData::not_applicable(test_filter));
        }
    }
    json_utils::write_json::<Vec<DecisionData>>(
        &format!(
            "{}/{}{}",
            &configuration.path_output_dir, DECISION_DATA_FILENAME, JSON_EXTENSION
        ),
        &diff_xjoules_data.decisions,
    );
}

#[cfg(test)]
mod test {
    use crate::fr_davidson_diff_xjoules::{
        configuration::Configuration,
        diff_data::{Decision, DiffXJoulesData},
        measure::version_measure::VersionMeasure,
        steps::test_selection::TestSelection,
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
            test_filters: vec![TestFilterEnum::All],
            mark_strategies: vec![MarkStrategyEnum::Strict],
            indicator_to_consider_for_marking: String::from("UNHALTED_REFERENCE_CYCLES"),
        };
        let mut data = DiffXJoulesData::new();
        data.test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        data.delta = json_utils::read_json::<VersionMeasure>("test_resources/delta.json");
        run(&configuration, &mut data);
        let decision_data = data.decisions;
        assert_eq!(4, decision_data[0].test_selection.test_selection.len());
        assert!(decision_data[0]
            .test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testAddedStatement"));
        assert!(decision_data[0]
            .test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testAddedAndRemovedStatement"));
        assert!(decision_data[0]
            .test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testUpdatedStatement"));
        assert!(decision_data[0]
            .test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testRemovedStatement"));
        match decision_data[0].decision {
            Decision::Pass => assert!(false),
            Decision::Break => assert!(true),
            Decision::NotApplicable => assert!(false),
        }
    }
}
