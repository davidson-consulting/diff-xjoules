use std::{
    collections::{HashMap, HashSet},
    fs,
};

use serde_derive::{Deserialize, Serialize};

use crate::fr_davidson_diff_xjoules::{
    utils::{
        command::run_command_redirect_to_file,
        coverage::{run_coverage_cmd, Coverage, COVERAGE_FILENAME},
        json_utils::{self, JSON_EXTENSION},
    },
    Configuration, DiffXJoulesData, SUFFIX_V1, SUFFIX_V2,
};

const DIFF_FILENAME: &str = "diff";
pub const TEST_SELECTION_FILENAME: &str = "test_selection";

#[derive(Serialize, Deserialize)]
pub struct TestSelection {
    pub test_selection: HashSet<String>,
}

impl TestSelection {
    pub fn new() -> TestSelection {
        TestSelection {
            test_selection: (HashSet::new()),
        }
    }
}

pub fn run(
    configuration: &Configuration,
    mut diff_xjoules_data: DiffXJoulesData,
) -> DiffXJoulesData {
    diff_xjoules_data.coverage_v1 = compute_coverage(
        &configuration.path_v1,
        &configuration.coverage_cmd,
        &configuration.path_output_dir,
        SUFFIX_V1,
    );
    diff_xjoules_data.coverage_v2 = compute_coverage(
        &configuration.path_v2,
        &configuration.coverage_cmd,
        &configuration.path_output_dir,
        SUFFIX_V2,
    );
    diff_xjoules_data.diff = compute_diff(
        &configuration.path_v1,
        &configuration.path_v2,
        &configuration.src_folder,
        &configuration.path_output_dir,
    );
    diff_xjoules_data = select_tests(&configuration.path_v1, diff_xjoules_data);
    json_utils::write_json(
        &format!(
            "{}/{}{}",
            &configuration.path_output_dir, TEST_SELECTION_FILENAME, JSON_EXTENSION
        ),
        &diff_xjoules_data.test_selection,
    );
    return diff_xjoules_data;
}

fn compute_coverage(
    path_to_project: &str,
    coverage_cmd: &str,
    path_output_dir: &str,
    suffix_version: &str,
) -> Coverage {
    return run_coverage_cmd(
        path_to_project,
        coverage_cmd,
        &format!(
            "{}/{}{}{}",
            path_output_dir, COVERAGE_FILENAME, suffix_version, JSON_EXTENSION
        ),
    );
}

fn compute_diff(path_v1: &str, path_v2: &str, src_folder: &str, path_output_dir: &str) -> String {
    run_command_redirect_to_file(
        &format!(
            "diff -r {}/{} {}/{}",
            path_v1, src_folder, path_v2, src_folder
        ),
        &format!("{}/{}", path_output_dir, DIFF_FILENAME),
    );
    return fs::read_to_string(format!("{}/{}", path_output_dir, DIFF_FILENAME)).unwrap();
}

fn select_tests(path_to_project: &str, mut diff_xjoules_data: DiffXJoulesData) -> DiffXJoulesData {
    let coverage_v1 = &diff_xjoules_data.coverage_v1;
    let coverage_v2 = &diff_xjoules_data.coverage_v2;
    let diff = &diff_xjoules_data.diff;
    let mut test_selection: TestSelection = TestSelection::new();
    let lines_diff = Vec::from_iter(diff.lines());
    let mut i = 0;
    let mut total_nb_modified_lines = 0;
    while i < lines_diff.len() {
        let filename =
            &lines_diff[i].split_whitespace().nth(2).unwrap()[path_to_project.len() + 1..];
        i = i + 1;
        loop {
            let operation = lines_diff[i];
            i = i + 1;
            let mut nb_modified_line: i32 = 0;
            while i < lines_diff.len()
                && (lines_diff[i].starts_with(">") || lines_diff[i].starts_with("<"))
            {
                nb_modified_line = nb_modified_line + 1;
                total_nb_modified_lines = total_nb_modified_lines + 1;
                i = i + 1;
            }
            let mut selected_tests: HashSet<String> = HashSet::<String>::new();
            if operation.contains("a") {
                selected_tests =
                    handle_diff_operation(operation, "a", nb_modified_line, filename, &coverage_v2);
            } else if operation.contains("d") {
                selected_tests =
                    handle_diff_operation(operation, "d", nb_modified_line, filename, &coverage_v1);
            } else if operation.contains("c") {
                selected_tests =
                    handle_diff_operation(operation, "c", nb_modified_line, filename, &coverage_v1);
                selected_tests.extend(handle_diff_operation(
                    operation,
                    "c",
                    nb_modified_line,
                    filename,
                    &coverage_v2,
                ));
            }
            if !selected_tests.is_empty() {
                diff_xjoules_data.nb_modified_lines_exec_per_test_identifier =
                    update_nb_modified_lines_exec_per_test_identifier(
                        diff_xjoules_data.nb_modified_lines_exec_per_test_identifier,
                        &selected_tests,
                        nb_modified_line,
                    );
                test_selection.test_selection.extend(selected_tests);
            }
            if i >= lines_diff.len() || lines_diff[i].starts_with("diff -r") {
                break;
            }
        }
    }
    diff_xjoules_data.nb_total_nb_modified_lines = total_nb_modified_lines;
    diff_xjoules_data.test_selection = test_selection;
    return diff_xjoules_data;
}

fn update_nb_modified_lines_exec_per_test_identifier(
    mut nb_modified_lines_exec_per_test_identifier: HashMap<String, i32>,
    selected_tests: &HashSet<String>,
    nb_modified_line: i32,
) -> HashMap<String, i32> {
    selected_tests.iter().for_each(|selected_test| {
        if !nb_modified_lines_exec_per_test_identifier.contains_key(selected_test) {
            nb_modified_lines_exec_per_test_identifier.insert(selected_test.clone(), 0);
        }
        nb_modified_lines_exec_per_test_identifier.insert(
            selected_test.clone(),
            nb_modified_lines_exec_per_test_identifier
                .get(selected_test)
                .unwrap()
                .clone()
                + nb_modified_line,
        );
    });
    return nb_modified_lines_exec_per_test_identifier;
}

fn handle_diff_operation(
    full_operation: &str,
    operation: &str,
    nb_modified_line: i32,
    filename: &str,
    coverage: &Coverage,
) -> HashSet<String> {
    let starting_line = full_operation
        .split(operation)
        .nth(0)
        .unwrap()
        .parse()
        .unwrap();
    let modified_lines: Vec<i32> = (starting_line..starting_line + nb_modified_line).collect();
    return coverage.find_test_executing_lines(filename, &modified_lines);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::{
        steps::test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
        utils::{
            command::run_command,
            json_utils::{self, read_json, JSON_EXTENSION},
        },
    };

    #[test]
    fn test_run() {
        run_command("cp test_resources/coverage_v2.json target/coverage_v2.json");
        let configuration = Configuration {
            path_v1: String::from("diff-jjoules/src/test/resources/diff-jjoules-toy-java-project"),
            path_v2: String::from(
                "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2",
            ),
            src_folder: String::from("src/main/java"),
            path_output_dir: String::from("target"),
            coverage_cmd: String::from(
                "cp test_resources/coverage_v1.json target/coverage_v1.json",
            ),
            instrumentation_cmd: String::from(""),
            execution_cmd: String::from(""),
            iteration_warmup: 0,
            iteration_run: 0,
            time_to_wait_in_millis: 0,
            test_filter: TestFilterEnum::All,
            mark_strategy: MarkStrategyEnum::Strict,
            indicator_to_consider_for_marking: String::from("cycles"),
        };
        let diff_xjoules_data = run(&configuration, DiffXJoulesData::new());
        let expected_diff_content = fs::read_to_string("test_resources/expected_diff").unwrap();
        assert_eq!(9, diff_xjoules_data.coverage_v1.test_coverages.len());
        assert_eq!(9, diff_xjoules_data.coverage_v2.test_coverages.len());
        assert_eq!(expected_diff_content, diff_xjoules_data.diff);
        assert_eq!(4, diff_xjoules_data.test_selection.test_selection.len());
        let test_selection = read_json::<TestSelection>(&format!(
            "{}/{}{}",
            "target", TEST_SELECTION_FILENAME, JSON_EXTENSION
        ));
        assert_eq!(4, test_selection.test_selection.len());
    }

    #[test]
    fn test_compute_coverage() {
        let coverage = compute_coverage(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/",
            "cp test_resources/coverage_v1.json target/coverage_v1.json",
            "target/",
            SUFFIX_V1,
        );
        assert_eq!(9, coverage.test_coverages.len());
        assert!(coverage
            .test_coverages
            .iter()
            .any(|test_coverage| test_coverage.test_identifier
                == "fr.davidson.AppTest#testRandomQuickSort"));
    }

    #[test]
    fn test_compute_diff() {
        compute_diff(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project",
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2",
            "src/main/java",
            "target",
        );
        let expected_diff_content = fs::read_to_string("test_resources/expected_diff").unwrap();
        let diff_content = fs::read_to_string(format!("{}/{}", "target", DIFF_FILENAME)).unwrap();
        assert_eq!(expected_diff_content, diff_content);
    }

    #[test]
    fn test_select_tests() {
        let mut diff_xjoules_data = DiffXJoulesData::new();
        diff_xjoules_data.coverage_v1 =
            json_utils::read_json::<Coverage>("test_resources/coverage_v1.json");
        diff_xjoules_data.coverage_v2 =
            json_utils::read_json::<Coverage>("test_resources/coverage_v2.json");
        diff_xjoules_data.diff = fs::read_to_string("test_resources/expected_diff").unwrap();
        let diff_xjoules_data = select_tests(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project",
            diff_xjoules_data,
        );
        let test_selection = diff_xjoules_data.test_selection;
        assert_eq!(4, test_selection.test_selection.len());
        assert!(test_selection
            .test_selection
            .contains(&String::from("fr.davidson.AppTest#testAddedStatement")));
        assert!(test_selection
            .test_selection
            .contains(&String::from("fr.davidson.AppTest#testRemovedStatement")));
        assert!(test_selection
            .test_selection
            .contains(&String::from("fr.davidson.AppTest#testUpdatedStatement")));
        assert!(test_selection.test_selection.contains(&String::from(
            "fr.davidson.AppTest#testAddedAndRemovedStatement"
        )));
        assert_eq!(12, diff_xjoules_data.nb_total_nb_modified_lines);
        assert_eq!(
            1,
            *diff_xjoules_data
                .nb_modified_lines_exec_per_test_identifier
                .get("fr.davidson.AppTest#testAddedStatement")
                .unwrap()
        );
        assert_eq!(
            1,
            *diff_xjoules_data
                .nb_modified_lines_exec_per_test_identifier
                .get("fr.davidson.AppTest#testRemovedStatement")
                .unwrap()
        );
        assert_eq!(
            1,
            *diff_xjoules_data
                .nb_modified_lines_exec_per_test_identifier
                .get("fr.davidson.AppTest#testUpdatedStatement")
                .unwrap()
        );
        assert_eq!(
            2,
            *diff_xjoules_data
                .nb_modified_lines_exec_per_test_identifier
                .get("fr.davidson.AppTest#testAddedAndRemovedStatement")
                .unwrap()
        );
    }
}
