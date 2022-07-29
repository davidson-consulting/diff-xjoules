use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration,
    diff_data::DiffXJoulesData,
    measure::{data::Data, version_measure::VersionMeasure},
    steps::test_selection::TestSelection,
    utils::coverage::Coverage,
};

use super::MarkStrategy;

pub struct CodeCoverageMarkStrategy {}

impl CodeCoverageMarkStrategy {
    fn get_considered_data(
        &self,
        selected_test: &str,
        data: &VersionMeasure,
        indicator_to_consider_for_marking: &str,
    ) -> f64 {
        let test_data_v1 = data.find_test_measure(selected_test).unwrap();
        return test_data_v1.get_median(indicator_to_consider_for_marking);
    }
    fn compute_weight(
        &self,
        coverage: &Coverage,
        selected_test: &str,
        total_nb_covered_lines: i32,
    ) -> f64 {
        let test_coverage_v1 = coverage.get_test_coverage_by_test_identifier(&selected_test);
        let nb_coverage_line_test_v1 = test_coverage_v1.get_total_nb_lines_covered();
        return (nb_coverage_line_test_v1 as f64) / (total_nb_covered_lines as f64);
    }
    fn compute_weighted_data(
        &self,
        coverage: &Coverage,
        selected_test: &str,
        total_nb_covered_lines: i32,
        indicator_to_consider_for_marking: &str,
        data: &VersionMeasure,
    ) -> f64 {
        let considered_test_data =
            self.get_considered_data(selected_test, &data, &indicator_to_consider_for_marking);
        let weight = self.compute_weight(&coverage, &selected_test, total_nb_covered_lines);
        return weight as f64 * considered_test_data;
    }
}

impl MarkStrategy for CodeCoverageMarkStrategy {
    fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool {
        let total_nb_covered_lines_v1 = data.coverage_v1.get_total_nb_lines_covered();
        let total_nb_covered_lines_v2 = data.coverage_v2.get_total_nb_lines_covered();
        return test_selection
            .test_selection
            .iter()
            .map(|selected_test| {
                let weighted_considered_test_data_v1 = self.compute_weighted_data(
                    &data.coverage_v1,
                    &selected_test,
                    total_nb_covered_lines_v1,
                    &configuration.indicator_to_consider_for_marking,
                    &data.data_v1,
                );
                let weighted_considered_test_data_v2 = self.compute_weighted_data(
                    &data.coverage_v2,
                    &selected_test,
                    total_nb_covered_lines_v2,
                    &configuration.indicator_to_consider_for_marking,
                    &data.data_v2,
                );
                return weighted_considered_test_data_v2 - weighted_considered_test_data_v1;
            })
            .sum::<f64>()
            < 0.0;
    }
}

impl CodeCoverageMarkStrategy {
    pub fn new() -> CodeCoverageMarkStrategy {
        CodeCoverageMarkStrategy {}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fr_davidson_diff_xjoules::{
        measure::test_measure::TestMeasure,
        steps::test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
        utils::json_utils,
    };

    #[test]
    fn test_get_considered_data() {
        let mut version_measure = VersionMeasure::new();
        let mut test_measure = TestMeasure::new("test");
        let mut data = Vec::<Data>::new();
        data.push(Data::new("cycles", 1000.0));
        data.push(Data::new("cycles", 1200.0));
        data.push(Data::new("not_used", 1.0));
        data.push(Data::new("not_used", 3.0));
        test_measure.measures.push(data);
        version_measure.test_measures.push(test_measure);
        let mut test_measure = TestMeasure::new("test2");
        let mut data = Vec::<Data>::new();
        data.push(Data::new("cycles", 10.0));
        data.push(Data::new("cycles", 12.0));
        data.push(Data::new("not_used", 1.0));
        data.push(Data::new("not_used", 3.0));
        test_measure.measures.push(data);
        version_measure.test_measures.push(test_measure);
        let code_coverage_mark_strategy = CodeCoverageMarkStrategy::new();
        assert_eq!(
            1100.0,
            code_coverage_mark_strategy.get_considered_data("test", &version_measure, "cycles")
        );
    }

    #[test]
    fn test_compute_weight() {
        let code_coverage_mark_strategy = CodeCoverageMarkStrategy::new();
        let coverage = json_utils::read_json::<Coverage>("test_resources/coverage_v1.json");
        assert_eq!(
            1.0,
            code_coverage_mark_strategy.compute_weight(
                &coverage,
                "fr.davidson.AppTest#testRandomQuickSortLarge",
                41,
            )
        );
        assert_eq!(
            0.5,
            code_coverage_mark_strategy.compute_weight(
                &coverage,
                "fr.davidson.AppTest#testRandomQuickSortLarge",
                82,
            )
        );
    }

    #[test]
    fn test_compute_weighted_data() {
        let coverage = json_utils::read_json::<Coverage>("test_resources/coverage_v1.json");
        let data = json_utils::read_json::<VersionMeasure>("test_resources/data_v1.json");
        let code_coverage_mark_strategy = CodeCoverageMarkStrategy::new();
        assert_eq!(
            21021.951219512197,
            code_coverage_mark_strategy.compute_weighted_data(
                &coverage,
                "fr.davidson.AppTest#testAddedStatement",
                41,
                "UNHALTED_REFERENCE_CYCLES",
                &data,
            )
        );
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
            test_filter: TestFilterEnum::All,
            mark_strategy: MarkStrategyEnum::CodeCov,
            indicator_to_consider_for_marking: String::from("UNHALTED_REFERENCE_CYCLES"),
        };
        let mut data = DiffXJoulesData::new();
        data.coverage_v1 = json_utils::read_json::<Coverage>("test_resources/coverage_v1.json");
        data.coverage_v2 = json_utils::read_json::<Coverage>("test_resources/coverage_v2.json");
        data.data_v1 = json_utils::read_json::<VersionMeasure>("test_resources/data_v1.json");
        data.data_v2 = json_utils::read_json::<VersionMeasure>("test_resources/data_v2.json");
        let test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        assert!(configuration
            .mark_strategy
            .decide(&configuration, &data, &test_selection));
    }
}
