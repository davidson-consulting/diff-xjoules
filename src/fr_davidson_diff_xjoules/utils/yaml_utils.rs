use serde::de::DeserializeOwned;
use std::fs;

pub fn read_yaml<T: DeserializeOwned>(path_to_yaml_file: std::path::PathBuf) -> T {
    let yaml_file_content = fs::read_to_string(path_to_yaml_file).unwrap();
    return serde_yaml::from_str::<T>(&yaml_file_content).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::configuration::Configuration;

    #[test]
    fn test_read_yaml() {
        let configuration = read_yaml::<Configuration>(std::path::PathBuf::from(
            "test_resources/configuration_file_example.yaml",
        ));
        assert_eq!(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project",
            configuration.path_v1
        );
        assert_eq!(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2",
            configuration.path_v2
        );
        assert_eq!("target", configuration.path_output_dir);
        assert_eq!("src/main/java", configuration.src_folder);
        assert_eq!("java -jar diff-jjoules/target/diff-jjoules-1.0.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project-v1 {{ path_project_v1 }} --path-to-project-v2 {{ path_project_v2 }} --second-path-to-project {{ second_path_project }} --task TEST_COVERAGE --output-path {{ output_path }}", configuration.coverage_cmd);
    }
}
