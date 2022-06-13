use std::fs;

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use serde_yaml;

pub fn read_json(path_to_json_file: String) -> Map<String, Value> {
    let json_file_content = fs::read_to_string(path_to_json_file).unwrap();
    let json_content: Value = serde_json::from_str(&json_file_content).unwrap();
    return json_content.as_object().unwrap().clone();
}

pub fn read_yaml<T: DeserializeOwned>(path_to_yaml_file: String) -> T
{
    let yaml_file_content = fs::read_to_string(path_to_yaml_file).unwrap();
    return serde_yaml::from_str::<T>(&yaml_file_content).unwrap();
}


#[cfg(test)]
mod tests {
    use crate::fr_davidson_diff_xjoules::Configuration;
    use super::*;

    #[test]
    fn test_read_json() {
        let json_content: Map<String, Value> = read_json(String::from("test_resources/input.json"));
        assert!(json_content.contains_key("fr.davidson.AppTest#testRandomQuickSort"));
        assert!(json_content["fr.davidson.AppTest#testRandomQuickSort"].as_object().unwrap().contains_key("src/test/java/fr/davidson/AppTest.java"));
        let coverage: &Vec<Value> = json_content["fr.davidson.AppTest#testRandomQuickSort"]["src/test/java/fr/davidson/AppTest.java"].as_array().unwrap();
        assert!(coverage[0].is_number());
        assert_eq!(20, coverage.len());
    }

    #[test]
    fn test_read_yaml() {
        let configuration = read_yaml::<Configuration>(String::from("test_resources/configuration_file_example.yaml"));
        assert_eq!("/tmp/v1", configuration.path_v1);
        assert_eq!("/tmp/v2", configuration.path_v2);
        assert_eq!("java -jar my.jar --task coverage --path ${path_project}", configuration.cmd_coverage);
    }
}
