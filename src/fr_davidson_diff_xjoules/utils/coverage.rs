use std::collections::HashSet;

use serde_derive::Deserialize;
use handlebars::Handlebars;

use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::utils::{command, json_utils};

pub const COVERAGE_FILENAME: &str = "coverage";

#[derive(Deserialize)]
pub struct Coverage {
    pub test_coverages: Vec<TestCoverage>
}

#[derive(Deserialize)]
pub struct TestCoverage {
    pub test_identifier: String,
    pub file_coverages: Vec<FileCoverage>
}

#[derive(Deserialize)]
pub struct FileCoverage {
    pub filename: String,
    pub covered_lines: Vec<i16>
}

pub fn find_test_executing_lines(coverage: &Coverage, filename: &str, lines: &Vec<i16>) -> HashSet<String> {
    let mut tests = HashSet::new();
    for (_, test_coverage) in coverage.test_coverages.iter().enumerate() {
        if execute_lines_of_file(&test_coverage, filename, lines) {
            tests.insert(test_coverage.test_identifier.clone());
        }
    }
    return tests;
}

pub fn execute_lines_of_file(test_coverage: &TestCoverage, filename: &str, lines: &Vec<i16>) -> bool {
    for (_, file_coverage) in test_coverage.file_coverages.iter().enumerate() {
        if file_coverage.filename.eq(filename) && any_lines_is_covered(&file_coverage, lines) {
            return true;
        }
    }
    return false;
}

pub fn any_lines_is_covered(file_coverage: &FileCoverage, lines: &Vec<i16>) -> bool {
    return lines.iter().any(|line| file_coverage.covered_lines.contains(line));
}

pub fn run_coverage_cmd(path_to_project: &str, coverage_cmd: &str, output_path: String) -> Coverage {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("coverage_cmd", coverage_cmd)
        .unwrap();
    let mut data = HashMap::new();
    data.insert("path_project", path_to_project);
    data.insert("output_path", &output_path);
    command::run_command(&handlebars.render("coverage_cmd", &data).unwrap());
    return json_utils::read_json::<Coverage>(&output_path);
}


mod tests {
    use crate::fr_davidson_diff_xjoules::utils::json_utils::read_json;
    use super::*;

    #[test]
    fn test_run_coverage_cmd() {
        assert_eq!(0, 1);
    }

    #[test]
    fn test_find_test_executing_lines() {
        let coverage: Coverage = read_json::<Coverage>("test_resources/coverage_v1");
        let mut lines = Vec::new();
        lines.push(21);
        lines.push(22);
        lines.push(23);
        let mut selected_tests = find_test_executing_lines(&coverage, "src/main/java/fr/davidson/App.java", &lines);
        assert_eq!(2, selected_tests.len());
        assert!(selected_tests.contains("fr.davidson.AppTest#testRandomQuickSort"));
        assert!(selected_tests.contains("fr.davidson.AppTest#testRandomQuickSortLarge"));
        lines = Vec::new();
        lines.push(47);
        lines.push(51);
        lines.push(52);
        selected_tests = find_test_executing_lines(&coverage, "src/main/java/fr/davidson/App.java", &lines);
        assert!(selected_tests.contains("fr.davidson.AppTest#testAddedStatement"));
        assert!(selected_tests.contains("fr.davidson.AppTest#testRemovedStatement"));
        lines = Vec::new();
        lines.push(76);
        selected_tests = find_test_executing_lines(&coverage, "src/main/java/fr/davidson/App.java", &lines);
        assert_eq!(0, selected_tests.len());
    }

}