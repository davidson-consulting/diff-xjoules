use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::{
    measure::version_measure::VersionMeasure,
    utils::{
        command,
        json_utils::{self, read_json, JSON_EXTENSION},
    },
    Configuration, DiffXJoulesData,
};

use rand::Rng;
use std::{thread, time};

use super::test_selection::TEST_SELECTION_FILENAME;

pub fn run(configuration: &Configuration, diff_xjoules_data: &mut DiffXJoulesData) {
    let tests_set_path = format!(
        "{}/{}{}",
        configuration.path_output_dir, TEST_SELECTION_FILENAME, JSON_EXTENSION
    );
    let mut data_v1 = VersionMeasure {
        test_measures: Vec::new(),
    };
    let mut data_v2 = VersionMeasure {
        test_measures: Vec::new(),
    };
    let mut rng = rand::thread_rng();
    let time_to_wait = time::Duration::from_millis(configuration.time_to_wait_in_millis);

    warmup(configuration, &tests_set_path);
    thread::sleep(time_to_wait);
    for i in 0..configuration.iteration_run {
        if rng.gen_bool(0.5) {
            run_and_merge_for_version(
                &configuration.path_v1,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v1,
            );
            run_and_merge_for_version(
                &configuration.path_v2,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v2,
            );
        } else {
            run_and_merge_for_version(
                &configuration.path_v2,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v2,
            );
            run_and_merge_for_version(
                &configuration.path_v1,
                &tests_set_path,
                &configuration.execution_cmd,
                &mut data_v1,
            );
        }
        if i % 3 == 0 {
            thread::sleep(time_to_wait);
        }
    }
    json_utils::write_json::<VersionMeasure>(
        &format!("{}/data_v1.json", configuration.path_output_dir),
        &data_v1,
    );
    json_utils::write_json::<VersionMeasure>(
        &format!("{}/data_v2.json", configuration.path_output_dir),
        &data_v2,
    );
    diff_xjoules_data.data_v1 = data_v1;
    diff_xjoules_data.data_v2 = data_v2;
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

pub fn run_and_merge_for_version(
    path_project: &str,
    test_set_path: &str,
    instrumentation_cmd: &str,
    version_data: &mut VersionMeasure,
) {
    run_once_for_version(path_project, test_set_path, instrumentation_cmd);
    let data = read_json::<VersionMeasure>(&format!(
        "{}/diff-measurements/measurements.json",
        path_project
    ));
    version_data.merge(data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::steps::test_mark::{
        mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum,
    };
    use std::{fs, path::Path};

    #[test]
    fn test_run() {
        if Path::new("target/v1").exists() {
            fs::remove_dir_all("target/v1").unwrap();
            fs::remove_dir_all("target/v2").unwrap();
        }
        fs::create_dir("target/v1").unwrap();
        fs::create_dir("target/v2").unwrap();
        fs::create_dir_all("target/v1/diff-measurements").unwrap();
        fs::create_dir_all("target/v2/diff-measurements").unwrap();
        command::run_command("cp test_resources/measurements_v1.json target/v1/measurements.json");
        command::run_command("cp test_resources/measurements_v2.json target/v2/measurements.json");
        let configuration = Configuration {
            path_v1: String::from("target/v1"),
            path_v2: String::from("target/v2"),
            src_folder: String::from("src/main/java"),
            path_output_dir: String::from("target"),
            coverage_cmd: String::from(""),
            instrumentation_cmd: String::from(""),
            execution_cmd: String::from(format!(
                "cp {}/measurements.json {}/diff-measurements/measurements.json",
                "{{ path_project }}", "{{ path_project }}"
            )),
            iteration_warmup: 1,
            iteration_run: 10,
            time_to_wait_in_millis: 1,
            test_filters: vec![TestFilterEnum::All],
            mark_strategies: vec![MarkStrategyEnum::Strict],
            indicator_to_consider_for_marking: String::from("cycles"),
        };
        let diff_xjoules_data = &mut DiffXJoulesData::new();
        run(&configuration, diff_xjoules_data);
        assert_eq!(4, diff_xjoules_data.data_v1.test_measures.len());
        assert_eq!(
            10,
            diff_xjoules_data
                .data_v1
                .test_measures
                .get(0)
                .unwrap()
                .measures
                .len()
        );
        assert_eq!(4, diff_xjoules_data.data_v2.test_measures.len());
        assert_eq!(
            10,
            diff_xjoules_data
                .data_v2
                .test_measures
                .get(0)
                .unwrap()
                .measures
                .len()
        );
    }

    // #[ignore]
    // #[test]
    // fn test_run_integration_with_java() {
    //     run_test(|| {
    //         command::run_command("mvn clean package -DskipTests -f diff-jjoules/pom.xml");
    //         command::run_command_redirect_to_file("ls diff-jjoules/target", "target/list_files");
    //         let list_files = fs::read_to_string("target/list_files").unwrap();
    //         let jar_filename = list_files
    //             .lines()
    //             .find(|file| file.ends_with("SNAPSHOT-jar-with-dependencies.jar"))
    //             .unwrap();

    //         // copy pre-instrumented classes
    //         command::run_command("cp test_resources/AppTest.java.v1 diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/src/test/java/fr/davidson/AppTest.java");
    //         command::run_command("cp test_resources/AppTest.java.v2 diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2/src/test/java/fr/davidson/AppTest.java");
    //         command::run_command("cp test_resources/pom.xml diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/pom.xml");
    //         command::run_command("cp test_resources/pom.xml diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2/pom.xml");

    //         let configuration = Configuration {
    //             path_v1: String::from("diff-jjoules/src/test/resources/diff-jjoules-toy-java-project"),
    //             path_v2: String::from(
    //                 "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2",
    //             ),
    //             src_folder: String::from("src/main/java"),
    //             path_output_dir: String::from("target"),
    //             coverage_cmd: String::from(""),
    //             instrumentation_cmd: String::from(""),
    //             execution_cmd: String::from(format!("java -jar diff-jjoules/target/{} --path-to-project {} --task TEST_EXECUTION --tests-set {}" , jar_filename, "{{ path_project }}", "{{ tests_set_path }}")),
    //             iteration_warmup: 1,
    //             iteration_run: 3,
    //             time_to_wait_in_millis: 500,
    //             test_filter: TestFilterEnum::ALL,
    //             mark_strategy: MarkStrategyEnum::STRICT,
    //             indicator_to_consider_for_marking: String::from("cycles")
    //         };
    //         run(&configuration, &mut DiffXJoulesData::new());
    //         let data_v1 = json_utils::read_json::<VersionMeasure>("target/data_v1.json");
    //         let data_v2 = json_utils::read_json::<VersionMeasure>("target/data_v2.json");
    //         assert_eq!(4, data_v1.test_measures.len());
    //         assert_eq!(4, data_v2.test_measures.len());
    //         assert_eq!(3, data_v1.test_measures[0].measures.len());
    //         assert_eq!(3, data_v2.test_measures[0].measures.len());
    //         assert_eq!(8, data_v1.test_measures[0].measures[0].len());
    //         assert_eq!(8, data_v2.test_measures[0].measures[0].len());
    //     });
    // }

    // #[cfg(test)]
    // fn run_test<T>(test: T) -> ()
    // where
    //     T: FnOnce() -> () + panic::UnwindSafe,
    // {
    //     let result = panic::catch_unwind(|| test());
    //     teardown();
    //     assert!(result.is_ok())
    // }

    // #[cfg(test)]
    // fn teardown() {
    //     command::run_command("git checkout -- diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2/ diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/");
    // }
}
