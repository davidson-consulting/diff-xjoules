use std::fs;

use crate::fr_davidson_diff_xjoules::{Configuration, SUFFIX_V1, SUFFIX_V2, utils::{coverage::{COVERAGE_FILENAME, run_coverage_cmd}, command::{run_command, run_command_redirect_to_file}}, DiffXJoulesData};

const DIFF_FILENAME: &str = "diff";

pub struct TestSelection {
    pub test_selection: Vec<String>
}

pub fn run(configuration: Configuration, mut diff_xjoules_data: DiffXJoulesData) {
    // diff_xjoules_data = compute_coverages(&configuration, diff_xjoules_data);
    // diff_xjoules_data.diff = Some(compute_diff(configuration));
}

fn compute_coverage(configuration: &Configuration, mut diff_xjoules_data: DiffXJoulesData) -> DiffXJoulesData {
    diff_xjoules_data.coverage_v1 = Some(run_coverage_cmd(
        &configuration.path_v1, 
        &configuration.cmd_coverage,
        format!("{}/{}{}", &configuration.path_output_dir, COVERAGE_FILENAME, SUFFIX_V1)
    ));
    diff_xjoules_data.coverage_v2 = Some(run_coverage_cmd(
        &configuration.path_v2,
         &configuration.cmd_coverage,
         format!("{}/{}{}", &configuration.path_output_dir, COVERAGE_FILENAME, SUFFIX_V2)
    ));
    return diff_xjoules_data;
}

fn compute_diff(path_v1: &str, path_v2: &str, src_folder: &str, path_output_dir: &str) -> String {
    run_command_redirect_to_file(
        &format!("diff -r {}/{} {}/{}", path_v1, src_folder, path_v2, src_folder),
        &format!("{}/{}", path_output_dir, DIFF_FILENAME)
    );
    return fs::read_to_string(format!("{}/{}", path_output_dir, DIFF_FILENAME)).unwrap();
}

fn select_tests(mut diff_xjoules_data: DiffXJoulesData) -> DiffXJoulesData {
    return diff_xjoules_data;
}


mod tests {
    use super::*;

    #[test]
    fn test_run() {

    }

    #[test]
    fn test_compute_coverages() {
        
    }

    #[test]
    fn test_compute_diff() {
        compute_diff(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/",
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2/",
            "src/main/java",
            "target",
        );
        let expected_diff_content = "diff -r diff-jjoules/src/test/resources/diff-jjoules-toy-java-project//src/main/java/fr/davidson/App.java diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2//src/main/java/fr/davidson/App.java\n19c19\n<             Integer pivot = arr.get(0);\n---\n>             Integer pivot = arr.get(new Random().nextInt(arr.size()));\n";
        let diff_content = fs::read_to_string(format!("{}/{}", "target", DIFF_FILENAME)).unwrap();
        assert_eq!(expected_diff_content, diff_content);
    }

}
