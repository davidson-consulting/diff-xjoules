use std::{collections::HashSet, fs};

use serde_derive::{Deserialize, Serialize};

use crate::fr_davidson_diff_xjoules::{
    utils::{
        command::{run_command_redirect_to_file},
        coverage::{find_test_executing_lines, run_coverage_cmd, Coverage, COVERAGE_FILENAME},
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

pub fn run(configuration: &Configuration, diff_xjoules_data: &mut DiffXJoulesData) {
    diff_xjoules_data.coverage_v1 = Some(compute_coverage(
        &configuration.path_v1,
        &configuration.coverage_cmd,
        &configuration.path_output_dir,
        SUFFIX_V1,
    ));
    diff_xjoules_data.coverage_v2 = Some(compute_coverage(
        &configuration.path_v2,
        &configuration.coverage_cmd,
        &configuration.path_output_dir,
        SUFFIX_V2,
    ));
    diff_xjoules_data.diff = Some(compute_diff(
        &configuration.path_v1,
        &configuration.path_v2,
        &configuration.src_folder,
        &configuration.path_output_dir,
    ));
    diff_xjoules_data.test_selection = Some(select_tests(
        &configuration.path_v1,
        diff_xjoules_data.coverage_v1.as_ref().unwrap(),
        diff_xjoules_data.coverage_v2.as_ref().unwrap(),
        &diff_xjoules_data.diff.as_ref().unwrap(),
    ));
    json_utils::write_json(
        &format!(
            "{}/{}{}",
            &configuration.path_output_dir, TEST_SELECTION_FILENAME, JSON_EXTENSION
        ),
        &diff_xjoules_data.test_selection,
    );
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

fn select_tests(
    path_to_project: &str,
    coverage_v1: &Coverage,
    coverage_v2: &Coverage,
    diff: &str,
) -> TestSelection {
    let mut test_selection: TestSelection = TestSelection::new();
    let lines_diff = Vec::from_iter(diff.lines());
    let mut i = 0;
    while i < lines_diff.len() {
        let filename =
            &lines_diff[i].split_whitespace().nth(2).unwrap()[path_to_project.len() + 1..];
        i = i + 1;
        loop {
            let operation = lines_diff[i];
            i = i + 1;
            let mut nb_modified_line = 0;
            while i < lines_diff.len()
                && (lines_diff[i].starts_with(">") || lines_diff[i].starts_with("<"))
            {
                nb_modified_line = nb_modified_line + 1;
                i = i + 1;
            }
            if operation.contains("a") {
                test_selection.test_selection.extend(handle_diff_operation(
                    operation,
                    "a",
                    nb_modified_line,
                    filename,
                    &coverage_v2,
                ));
            } else if operation.contains("d") {
                test_selection.test_selection.extend(handle_diff_operation(
                    operation,
                    "d",
                    nb_modified_line,
                    filename,
                    &coverage_v1,
                ));
            } else if operation.contains("c") {
                test_selection.test_selection.extend(handle_diff_operation(
                    operation,
                    "c",
                    nb_modified_line,
                    filename,
                    &coverage_v1,
                ));
                test_selection.test_selection.extend(handle_diff_operation(
                    operation,
                    "c",
                    nb_modified_line,
                    filename,
                    &coverage_v2,
                ));
            }
            if i >= lines_diff.len() || lines_diff[i].starts_with("diff -r") {
                break;
            }
        }
    }
    return test_selection;
}

fn handle_diff_operation(
    full_operation: &str,
    operation: &str,
    nb_modified_line: i16,
    filename: &str,
    coverage: &Coverage,
) -> HashSet<String> {
    let starting_line = full_operation
        .split(operation)
        .nth(0)
        .unwrap()
        .parse()
        .unwrap();
    let modified_lines: Vec<i16> = (starting_line..starting_line + nb_modified_line).collect();
    return find_test_executing_lines(coverage, filename, &modified_lines);
}

mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::utils::{json_utils::{self, read_json, JSON_EXTENSION}, command::run_command};

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
            iteration_run: 0
        };
        let mut diff_xjoules_data = DiffXJoulesData::new();
        run(&configuration, &mut diff_xjoules_data);
        let expected_diff_content = fs::read_to_string("test_resources/expected_diff").unwrap();
        assert_eq!(
            9,
            diff_xjoules_data.coverage_v1.unwrap().test_coverages.len()
        );
        assert_eq!(
            9,
            diff_xjoules_data.coverage_v2.unwrap().test_coverages.len()
        );
        assert_eq!(expected_diff_content, diff_xjoules_data.diff.unwrap());
        assert_eq!(
            4,
            diff_xjoules_data
                .test_selection
                .unwrap()
                .test_selection
                .len()
        );
        let test_selection =
            read_json::<TestSelection>(&format!("{}/{}{}", "target", TEST_SELECTION_FILENAME, JSON_EXTENSION));
        assert_eq!(4, test_selection.test_selection.len());
    }

    #[test]
    fn test_compute_coverage() {
        let coverage = compute_coverage(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/",
            "cp test_resources/coverage.json target/coverage_v1.json",
            "target/",
            SUFFIX_V1,
        );
        assert_eq!(9, coverage.test_coverages.len());
        assert!(coverage.test_coverages.iter().any(|test_coverage| test_coverage.test_identifier == "fr.davidson.AppTest#testRandomQuickSort"));
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
        let coverage_v1: Coverage = json_utils::read_json::<Coverage>("test_resources/coverage_v1.json");
        let coverage_v2: Coverage = json_utils::read_json::<Coverage>("test_resources/coverage_v2.json");
        let diff = fs::read_to_string("test_resources/expected_diff").unwrap();
        let test_selection = select_tests(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project",
            &coverage_v1,
            &coverage_v2,
            &diff,
        );
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
    }
}
