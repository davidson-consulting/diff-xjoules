use std::fs;
use serde::de::DeserializeOwned;

pub fn read_yaml<T: DeserializeOwned>(path_to_yaml_file: String) -> T {
    let yaml_file_content = fs::read_to_string(path_to_yaml_file).unwrap();
    return serde_yaml::from_str::<T>(&yaml_file_content).unwrap();
}


#[cfg(test)]
mod tests {
    use crate::fr_davidson_diff_xjoules::Configuration;
    use super::*;

    #[test]
    fn test_read_yaml() {
        let configuration = read_yaml::<Configuration>(String::from("test_resources/configuration_file_example.yaml"));
        assert_eq!("/tmp/v1", configuration.path_v1);
        assert_eq!("/tmp/v2", configuration.path_v2);
        assert_eq!("java -jar my.jar --task coverage --path ${path_project}", configuration.cmd_coverage);
    }
}
