use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::{
    utils::{command, json_utils::JSON_EXTENSION},
    Configuration,
};

use super::test_selection::TEST_SELECTION_FILENAME;

pub fn run(configuration: &Configuration) {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("path_project", &configuration.path_v1);
    let tests_set_path = format!(
        "{}/{}{}",
        configuration.path_output_dir, TEST_SELECTION_FILENAME, JSON_EXTENSION
    );
    data.insert("tests_set_path", &tests_set_path);
    command::run_templated_command(&configuration.instrumentation_cmd, &data);
    data.insert("path_project", &configuration.path_v2);
    command::run_templated_command(&configuration.instrumentation_cmd, &data);
}

mod tests {
    use crate::fr_davidson_diff_xjoules::steps::test_mark::{test_filter::TestFilterEnum, mark_strategy::MarkStrategyEnum};

    use super::*;
    use std::{fs, panic};

    //#[test]
    fn test_run_integration_with_java() {
        run_test(|| {
            command::run_command("mvn clean package -DskipTests -f diff-jjoules/pom.xml");
            command::run_command_redirect_to_file("ls diff-jjoules/target", "target/list_files");
            let list_files = fs::read_to_string("target/list_files").unwrap();
            let jar_filename = list_files.lines().find(|file| file.ends_with("SNAPSHOT-jar-with-dependencies.jar")).unwrap();
            let configuration = Configuration {
                path_v1: String::from("diff-jjoules/src/test/resources/diff-jjoules-toy-java-project"),
                path_v2: String::from("diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2"),
                src_folder: String::from("src/main/java"),
                path_output_dir: String::from("target"),
                coverage_cmd: String::from(""),
                instrumentation_cmd: String::from(format!("java -jar diff-jjoules/target/{} --path-to-project {} --task TEST_INSTRUMENTATION --tests-set {}", jar_filename, "{{ path_project }}", "{{ tests_set_path }}")),
                execution_cmd: String::from(""),
                iteration_warmup: 0,
                iteration_run: 3,
                time_to_wait_in_millis: 0,
                test_filter: TestFilterEnum::ALL,
                mark_strategy: MarkStrategyEnum::STRICT,
                indicator_to_consider_for_marking: String::from("cycles")
            };
            run(&configuration);
            command::run_command_redirect_to_file("git diff", "target/diff_diff-xjoules");
            let diff_xjoules_diff = fs::read_to_string("target/diff_diff-xjoules").unwrap();
            assert!(diff_xjoules_diff.lines().any(|line| line.contains("diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/src/main/java/fr/davidson/App.java")));
        })
    }
    
    fn run_test<T>(test: T) -> ()
        where T: FnOnce() -> () + panic::UnwindSafe
    {
        let result = panic::catch_unwind(|| {
            test()
        });
        teardown();
        assert!(result.is_ok())
    }

    fn teardown() {
        command::run_command("git checkout -- diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2/ diff-jjoules/src/test/resources/diff-jjoules-toy-java-project/");
    }

}
