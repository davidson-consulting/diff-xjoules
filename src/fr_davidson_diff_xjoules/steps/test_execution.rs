use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::{
    utils::{
        command,
        json_utils::{read_json, JSON_EXTENSION},
    },
    Configuration, DiffXJoulesData, VersionMeasure,
};

use rand::Rng;

use super::test_selection::TEST_SELECTION_FILENAME;

pub fn run(configuration: &Configuration, mut diff_xjoules_data: DiffXJoulesData) {
    let tests_set_path = format!(
        "{}/{}{}",
        configuration.path_output_dir, TEST_SELECTION_FILENAME, JSON_EXTENSION
    );
    let mut data_v1 = VersionMeasure { test_measures: Vec::new() };
    let mut data_v2 = VersionMeasure { test_measures: Vec::new() };
    let mut rng = rand::thread_rng();
    
    warmup(configuration, &tests_set_path);
    for i in 0..configuration.iteration_run {
        if rng.gen_bool(0.5) {
            run_and_merge_for_version(
                &configuration.path_v1,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v1
            );
            run_and_merge_for_version(
                &configuration.path_v2,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v2
            );
        } else {
            run_and_merge_for_version(
                &configuration.path_v2,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v2
            );
            run_and_merge_for_version(
                &configuration.path_v1,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v1
            );
        }
        if i % 3 == 0 {
            println!("CHILL");
        }
    }
    diff_xjoules_data.data_v1 = Some(data_v1);
    diff_xjoules_data.data_v2 = Some(data_v2);
}

pub fn warmup(configuration: &Configuration, tests_set_path: &str) {
    for _ in 0..configuration.iteration_warmup {
        run_once_for_version(
            &configuration.path_v1,
            &tests_set_path,
            &configuration.execution_cmd,
        );
        run_once_for_version(
            &configuration.path_v2,
            &tests_set_path,
            &configuration.execution_cmd,
        );
    }
}

pub fn run_once_for_version(path_project: &str, test_set_path: &str, instrumentation_cmd: &str) {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("path_project", path_project);
    data.insert("tests_set_path", test_set_path);
    command::run_templated_command(instrumentation_cmd, &data);
}

pub fn run_and_merge_for_version(path_project: &str, test_set_path: &str, instrumentation_cmd: &str, version_data: & mut VersionMeasure) {
    run_once_for_version(path_project, test_set_path, instrumentation_cmd);
    let data = read_json::<VersionMeasure>(&format!("{}/diff-measurements/measurements.json", path_project));
    version_data.merge(data);
}

mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::utils::{json_utils::{self, read_json, JSON_EXTENSION}, command::run_command};

    #[test]
    fn test_run() {
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
            execution_cmd: String::from("java -jar diff-jjoules/target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project {{ path_project }} --task TEST_EXECUTION --tests-set {{ tests_set_path }}"),
            iteration_warmup: 0,
            iteration_run: 3
        };
        run(&configuration, DiffXJoulesData::new());
    }
}
