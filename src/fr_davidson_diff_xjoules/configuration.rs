use serde_derive::Deserialize;

use super::{
    steps::test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
    utils::yaml_utils::read_yaml,
};

#[derive(Deserialize)]
pub struct Configuration {
    pub path_v1: String,
    pub path_v2: String,
    pub src_folder: String,
    pub path_output_dir: String,
    pub coverage_cmd: String,
    pub instrumentation_cmd: String,
    pub execution_cmd: String,
    pub iteration_warmup: i8,
    pub iteration_run: i8,
    pub time_to_wait_in_millis: u64,
    pub test_filter: TestFilterEnum,
    pub mark_strategy: MarkStrategyEnum,
    pub indicator_to_consider_for_marking: String,
}

impl Configuration {
    pub fn new(path_to_configuration_file: std::path::PathBuf) -> Configuration {
        return read_yaml::<Configuration>(path_to_configuration_file);
    }
}

#[cfg(test)]
mod test {
    use super::Configuration;

    #[test]
    fn test_new() {
        let configuration = Configuration::new(std::path::PathBuf::from(
            "test_resources/configuration_file_example.yaml",
        ));
        // Partial checking to not failing (at least less often) when we modify the test resources
        assert_eq!(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project",
            configuration.path_v1
        );
        assert_eq!(
            "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2",
            configuration.path_v2
        );
        assert_eq!("target", configuration.path_output_dir);
        assert_eq!(
            "RAPL_ENERGY_PKG",
            configuration.indicator_to_consider_for_marking
        );
    }
}
