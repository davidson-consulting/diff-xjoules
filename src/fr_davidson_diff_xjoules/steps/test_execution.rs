use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::{
    utils::{
        command,
        json_utils::{read_json, JSON_EXTENSION},
    },
    Configuration, DiffXJoulesData, VersionMeasure,
};

use super::test_selection::TEST_SELECTION_FILENAME;

pub fn run(configuration: &Configuration, mut diff_xjoules_data: DiffXJoulesData) {
    let tests_set_path = format!(
        "{}/{}{}",
        configuration.path_output_dir, TEST_SELECTION_FILENAME, JSON_EXTENSION
    );
    diff_xjoules_data.data_v1 = Some(VersionMeasure {
        test_measures: Vec::new(),
    });
    diff_xjoules_data.data_v2 = Some(VersionMeasure {
        test_measures: Vec::new(),
    });
    // WARM UP
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
    for i in 0..configuration.iteration_run {
        run_once_for_version(
            &configuration.path_v1,
            &tests_set_path,
            &configuration.execution_cmd,
        );
        diff_xjoules_data.data_v1 = Some(merge_test_measure(
            &configuration.path_v1,
            diff_xjoules_data.data_v1.unwrap(),
        ));

        run_once_for_version(
            &configuration.path_v2,
            &tests_set_path,
            &configuration.execution_cmd,
        );
        diff_xjoules_data.data_v2 = Some(merge_test_measure(
            &configuration.path_v2,
            diff_xjoules_data.data_v2.unwrap(),
        ));

        if i % 3 == 0 {
            println!("CHILL");
        }
    }
}

pub fn merge_test_measure(path_project: &str, mut data_version: VersionMeasure) -> VersionMeasure {
    let data = read_json::<VersionMeasure>(&format!("{}/measurements.json", path_project));
    for test_measure in data.test_measures.into_iter() {
        data_version
            .test_measures
            .iter_mut()
            .find(|test_measure_to_update| {
                test_measure_to_update.test_identifier == test_measure.test_identifier
            })
            .unwrap()
            .measures
            .extend(test_measure.measures);
    }
    return data_version;
}

pub fn run_once_for_version(path_project: &str, test_set_path: &str, instrumentation_cmd: &str) {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("path_project", path_project);
    data.insert("tests_set_path", test_set_path);
    command::run_templated_command(instrumentation_cmd, &data);
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
