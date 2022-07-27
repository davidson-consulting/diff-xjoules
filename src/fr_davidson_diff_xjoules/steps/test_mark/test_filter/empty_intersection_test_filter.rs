use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use super::TestFilter;

pub struct EmptyIntersectionTestFilter {}

impl TestFilter for EmptyIntersectionTestFilter {
    fn filter(&self, configuration: &Configuration, data: &DiffXJoulesData) -> TestSelection {
        let mut test_selection = TestSelection::new();
        for selected_test in data.test_selection.test_selection.iter() {
            let test_measure_v1 = data.data_v1.find_test_measure(&selected_test).unwrap();
            let test_measure_v2 = data.data_v2.find_test_measure(&selected_test).unwrap();
            let min_v1 = test_measure_v1.get_min(&configuration.indicator_to_consider_for_marking);
            let min_v2 = test_measure_v2.get_min(&configuration.indicator_to_consider_for_marking);
            let max_v1 = test_measure_v1.get_max(&configuration.indicator_to_consider_for_marking);
            let max_v2 = test_measure_v2.get_max(&configuration.indicator_to_consider_for_marking);
            if min_v1 > max_v2 || max_v1 < min_v2 {
                test_selection
                    .test_selection
                    .insert(selected_test.to_string());
            }
        }
        self.report(configuration, &test_selection);
        return test_selection;
    }

    fn report(&self, configuration: &Configuration, test_selection: &TestSelection) {
        crate::fr_davidson_diff_xjoules::utils::json_utils::write_json::<TestSelection>(
            &format!(
                "{}/test_filter_selection.json",
                configuration.path_output_dir
            ),
            &test_selection,
        );
    }
}

impl EmptyIntersectionTestFilter {
    pub fn new() -> EmptyIntersectionTestFilter {
        EmptyIntersectionTestFilter {}
    }
}

#[cfg(test)]
mod test {
    use crate::fr_davidson_diff_xjoules::{
        configuration::Configuration,
        diff_data::DiffXJoulesData,
        measure::{data::Data, test_measure::TestMeasure, version_measure::VersionMeasure},
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
            test_filter: TestFilterEnum::EmptyIntersection,
            mark_strategy: MarkStrategyEnum::Strict,
            indicator_to_consider_for_marking: String::from("cycles"),
        };
        let mut test_selection = TestSelection::new();
        let mut data = DiffXJoulesData::new();

        let mut version_measure_1 = VersionMeasure::new();
        let mut version_measure_2 = VersionMeasure::new();

        let mut measures = Vec::<f64>::new();
        measures.push(100.0);
        measures.push(90.0);
        measures.push(95.0);
        version_measure_1.test_measures.push(build_test_measure(
            "test_min_v1_gt_max_v2",
            "cycles",
            measures,
        ));

        let mut measures = Vec::<f64>::new();
        measures.push(70.0);
        measures.push(60.0);
        measures.push(65.0);
        version_measure_2.test_measures.push(build_test_measure(
            "test_min_v1_gt_max_v2",
            "cycles",
            measures,
        ));
        test_selection
            .test_selection
            .insert(String::from("test_min_v1_gt_max_v2"));

        let mut measures = Vec::<f64>::new();
        measures.push(70.0);
        measures.push(60.0);
        measures.push(65.0);
        version_measure_1.test_measures.push(build_test_measure(
            "test_max_v1_lt_min_v2",
            "cycles",
            measures,
        ));

        let mut measures = Vec::<f64>::new();
        measures.push(100.0);
        measures.push(90.0);
        measures.push(95.0);
        version_measure_2.test_measures.push(build_test_measure(
            "test_max_v1_lt_min_v2",
            "cycles",
            measures,
        ));
        test_selection
            .test_selection
            .insert(String::from("test_max_v1_lt_min_v2"));

        let mut measures = Vec::<f64>::new();
        measures.push(70.0);
        measures.push(60.0);
        measures.push(65.0);
        version_measure_1.test_measures.push(build_test_measure(
            "test_max_v1_lt_max_v2_and_min_v1_gt_min_v2",
            "cycles",
            measures,
        ));

        let mut measures = Vec::<f64>::new();
        measures.push(100.0);
        measures.push(70.0);
        measures.push(40.0);
        version_measure_2.test_measures.push(build_test_measure(
            "test_max_v1_lt_max_v2_and_min_v1_gt_min_v2",
            "cycles",
            measures,
        ));
        test_selection
            .test_selection
            .insert(String::from("test_max_v1_lt_max_v2_and_min_v1_gt_min_v2"));

        let mut measures = Vec::<f64>::new();
        measures.push(100.0);
        measures.push(70.0);
        measures.push(40.0);
        version_measure_1.test_measures.push(build_test_measure(
            "test_max_v1_gt_max_v2_and_min_v1_lt_min_v2",
            "cycles",
            measures,
        ));

        let mut measures = Vec::<f64>::new();
        measures.push(70.0);
        measures.push(60.0);
        measures.push(65.0);
        version_measure_2.test_measures.push(build_test_measure(
            "test_max_v1_gt_max_v2_and_min_v1_lt_min_v2",
            "cycles",
            measures,
        ));
        test_selection
            .test_selection
            .insert(String::from("test_max_v1_gt_max_v2_and_min_v1_lt_min_v2"));

        data.data_v1 = version_measure_1;
        data.data_v2 = version_measure_2;
        data.test_selection = test_selection;
        json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        let filter_test_selection = configuration.test_filter.filter(&configuration, &data);
        assert_eq!(2, filter_test_selection.test_selection.len());
    }

    fn build_test_measure(test_identifier: &str, indicator: &str, values: Vec<f64>) -> TestMeasure {
        let mut test_measure = TestMeasure::new(test_identifier);
        let mut data = Vec::<Data>::new();
        values
            .iter()
            .for_each(|value| data.push(Data::new(indicator, *value)));
        test_measure.measures.push(data);
        return test_measure;
    }
}
