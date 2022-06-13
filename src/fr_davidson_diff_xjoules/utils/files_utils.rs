use std::fs;

use serde::de::DeserializeOwned;
use serde_yaml;

pub fn read_json<T: DeserializeOwned>(path_to_json_file: String) -> T {
    let json_file_content = fs::read_to_string(path_to_json_file).unwrap();
    return serde_json::from_str::<T>(&json_file_content).unwrap();
}

pub fn read_yaml<T: DeserializeOwned>(path_to_yaml_file: String) -> T {
    let yaml_file_content = fs::read_to_string(path_to_yaml_file).unwrap();
    return serde_yaml::from_str::<T>(&yaml_file_content).unwrap();
}


#[cfg(test)]
mod tests {
    use crate::fr_davidson_diff_xjoules::{Configuration, utils::Coverage};
    use super::*;

    #[test]
    fn test_read_json() {
        let coverage: Coverage = read_json::<Coverage>(String::from("test_resources/coverage.json"));
        assert_eq!("fr.davidson.AppTest#testRandomQuickSort", coverage.test_coverages.get(0).unwrap().test_identifier);
        assert_eq!("src/main/java/fr/davidson/App.java", coverage.test_coverages.get(0).unwrap().file_coverages.get(0).unwrap().filename);
        assert_eq!(21, coverage.test_coverages.get(0).unwrap().file_coverages.get(0).unwrap().covered_lines.len());
    }

    #[test]
    fn test_read_yaml() {
        let configuration = read_yaml::<Configuration>(String::from("test_resources/configuration_file_example.yaml"));
        assert_eq!("/tmp/v1", configuration.path_v1);
        assert_eq!("/tmp/v2", configuration.path_v2);
        assert_eq!("java -jar my.jar --task coverage --path ${path_project}", configuration.cmd_coverage);
    }
}
