use std::fs;
use serde_json::{Map, Value};

pub fn read_json(path_to_json_file: String) -> Map<String, Value> {
    let json_file_content = fs::read_to_string(path_to_json_file).unwrap();
    let json_content: Value = serde_json::from_str(&json_file_content).unwrap();
    return json_content.as_object().unwrap().clone();
}

#[cfg(test)]
mod tests {
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
}
