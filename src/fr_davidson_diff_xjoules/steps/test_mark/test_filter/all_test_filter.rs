use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_xjoules_data::DiffXJoulesData,
    steps::test_selection::TestSelection,
};

use super::TestFilter;

pub struct AllTestFilter {}

impl TestFilter for AllTestFilter {
    fn filter(&self, configuration: &Configuration, data: &DiffXJoulesData) -> TestSelection {
        let mut test_selection = TestSelection::new();
        for selected_test in data.test_selection.test_selection.iter() {
            test_selection
                .test_selection
                .insert(selected_test.to_string());
        }
        self.report(configuration, &test_selection);
        return test_selection;
    }
}

impl AllTestFilter {
    pub fn new() -> AllTestFilter {
        AllTestFilter {}
    }
}

#[cfg(test)]
mod test {
    use crate::fr_davidson_diff_xjoules::{
        configuration::Configuration,
        diff_xjoules_data::DiffXJoulesData,
        measure::version_measure::VersionMeasure,
        steps::{
            test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
            test_selection::TestSelection,
        },
        utils::json_utils,
    };

    #[test]
    fn test_filter() {
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
        data.delta = json_utils::read_json::<VersionMeasure>("test_resources/delta.json");
        data.test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        assert_eq!("all_test", configuration.test_filters[0].to_string());
        let test_selection = configuration.test_filters[0].filter(&configuration, &data);
        assert_eq!(4, test_selection.test_selection.len());
        assert!(test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testAddedStatement"));
        assert!(test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testAddedAndRemovedStatement"));
        assert!(test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testUpdatedStatement"));
        assert!(test_selection
            .test_selection
            .contains("fr.davidson.AppTest#testRemovedStatement"));
    }
}
