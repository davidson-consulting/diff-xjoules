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
    fn get_considered_data<'a>(
        &self,
        selected_test: &'a str,
        data: &'a VersionMeasure,
        indicator_to_consider_for_marking: &'a str,
    ) -> &'a Data {
        let test_data_v1 = data.find_test_measure(selected_test).unwrap();
        return test_data_v1.measures[0]
            .iter()
            .find(|data| data.indicator.eq(indicator_to_consider_for_marking))
            .unwrap();
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
        return weight as f64 * considered_test_data.value;
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