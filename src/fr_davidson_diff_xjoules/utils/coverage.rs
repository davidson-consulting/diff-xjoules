use std::collections::HashSet;

use serde_derive::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::utils::{command, json_utils};

pub const COVERAGE_FILENAME: &str = "coverage";

#[derive(Serialize, Deserialize)]
pub struct Coverage {
    pub test_coverages: Vec<TestCoverage>,
}

impl Coverage {
    pub fn find_test_executing_lines(&self, filename: &str, lines: &Vec<i32>) -> HashSet<String> {
        let mut tests = HashSet::new();
        for (_, test_coverage) in self.test_coverages.iter().enumerate() {
            if test_coverage.execute_lines_of_file(filename, lines) {
                tests.insert(test_coverage.test_identifier.clone());
            }
        }
        return tests;
    }
    pub fn get_total_nb_lines_covered(&self) -> i32 {
        let mut covered_lines_in_filename = HashSet::<String>::new();
        for test_coverage in self.test_coverages.iter() {
            for file_coverage in test_coverage.file_coverages.iter() {
                for covered_line in file_coverage.covered_lines.iter() {
                    covered_lines_in_filename
                        .insert(file_coverage.filename.clone() + ":" + &covered_line.to_string());
                }
            }
        }
        return covered_lines_in_filename.len().try_into().unwrap();
    }
    pub fn get_test_coverage_by_test_identifier(&self, test_identifier: &str) -> &TestCoverage {
        return self
            .test_coverages
            .iter()
            .find(|test_coverage| test_coverage.test_identifier.eq(test_identifier))
            .unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestCoverage {
    pub test_identifier: String,
    pub file_coverages: Vec<FileCoverage>,
}

impl TestCoverage {
    pub fn execute_lines_of_file(&self, filename: &str, lines: &Vec<i32>) -> bool {
        for (_, file_coverage) in self.file_coverages.iter().enumerate() {
            if file_coverage.filename.eq(filename) && file_coverage.any_lines_is_covered(lines) {
                return true;
            }
        }
        return false;
    }
    pub fn get_total_nb_lines_covered(&self) -> i32 {
        return self
            .file_coverages
            .iter()
            .map(|file_coverage| file_coverage.covered_lines.len() as i32)
            .sum();
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileCoverage {
    pub filename: String,
    pub covered_lines: Vec<i32>,
}

impl FileCoverage {
    pub fn new(filename: &str) -> FileCoverage {
        return FileCoverage {
            filename: String::from(filename),
            covered_lines: Vec::<i32>::new(),
        };
    }
    pub fn any_lines_is_covered(&self, lines: &Vec<i32>) -> bool {
        return lines.iter().any(|line| self.covered_lines.contains(line));
    }
}

pub fn run_coverage_cmd(path_to_project: &str, coverage_cmd: &str, output_path: &str) -> Coverage {
    let mut data = HashMap::new();
    data.insert("path_project", path_to_project);
    data.insert("output_path", output_path);
    command::run_templated_command(coverage_cmd, &data);
    return json_utils::read_json::<Coverage>(output_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::utils::json_utils::read_json;

    #[test]
    fn test_test_coverage_get_total_nb_lines_covered() {
        let coverage: Coverage = read_json::<Coverage>("test_resources/coverage_v1.json");
        let test_coverage = coverage
            .get_test_coverage_by_test_identifier("fr.davidson.AppTest#testRandomQuickSortLarge");
        assert_eq!(41, test_coverage.get_total_nb_lines_covered());
    }

    #[test]
    fn test_get_test_coverage_by_test_identifier() {
        let coverage: Coverage = read_json::<Coverage>("test_resources/coverage_v1.json");
        let test_coverage = coverage
            .get_test_coverage_by_test_identifier("fr.davidson.AppTest#testRandomQuickSortLarge");
        assert_eq!(2, test_coverage.file_coverages.len());
        let file_coverage = test_coverage
            .file_coverages
            .iter()
            .find(|file_coverage| {
                file_coverage
                    .filename
                    .eq("src/main/java/fr/davidson/App.java")
            })
            .unwrap();
        assert_eq!(21, file_coverage.covered_lines.len());
    }

    #[test]
    fn test_new_file_coverage() {
        let file_coverage = FileCoverage::new("this_a_source_filename");
        assert_eq!("this_a_source_filename", file_coverage.filename);
        assert!(file_coverage.covered_lines.is_empty());
    }

    #[test]
    fn test_get_total_nb_lines_covered() {
        let coverage: Coverage = read_json::<Coverage>("test_resources/coverage_v1.json");
        assert_eq!(74, coverage.get_total_nb_lines_covered())
    }

    #[test]
    fn test_run_coverage_cmd() {
        let coverage = run_coverage_cmd(
            "target/",
            "cp test_resources/coverage_v1.json {{ output_path }}",
            "target/coverage.json",
        );
        assert_eq!(9, coverage.test_coverages.len());
    }

    #[test]
    fn test_find_test_executing_lines() {
        let coverage: Coverage = read_json::<Coverage>("test_resources/coverage_v1.json");
        let mut lines = Vec::new();
        lines.push(21);
        lines.push(22);
        lines.push(23);
        let mut selected_tests =
            coverage.find_test_executing_lines("src/main/java/fr/davidson/App.java", &lines);
        assert_eq!(2, selected_tests.len());
        assert!(selected_tests.contains("fr.davidson.AppTest#testRandomQuickSort"));
        assert!(selected_tests.contains("fr.davidson.AppTest#testRandomQuickSortLarge"));
        lines = Vec::new();
        lines.push(47);
        lines.push(51);
        lines.push(52);
        selected_tests =
            coverage.find_test_executing_lines("src/main/java/fr/davidson/App.java", &lines);
        assert!(selected_tests.contains("fr.davidson.AppTest#testAddedStatement"));
        assert!(selected_tests.contains("fr.davidson.AppTest#testRemovedStatement"));
        lines = Vec::new();
        lines.push(76);
        selected_tests =
            coverage.find_test_executing_lines("src/main/java/fr/davidson/App.java", &lines);
        assert_eq!(0, selected_tests.len());
    }
}
