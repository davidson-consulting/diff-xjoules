use std::fs;
use serde::de::DeserializeOwned;

const JSON_EXTENSION: &str = ".json";

pub fn read_json<T: DeserializeOwned>(path_to_json_file: &str) -> T {
    let json_file_content = fs::read_to_string(format!("{}{}", path_to_json_file, JSON_EXTENSION)).unwrap();
    return serde_json::from_str::<T>(&json_file_content).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::fr_davidson_diff_xjoules::utils::coverage::Coverage;
    use super::*;

    #[test]
    fn test_read_json() {
        let coverage: Coverage = read_json::<Coverage>("test_resources/coverage");
        assert_eq!("fr.davidson.AppTest#testRandomQuickSort", coverage.test_coverages.get(0).unwrap().test_identifier);
        assert_eq!("src/main/java/fr/davidson/App.java", coverage.test_coverages.get(0).unwrap().file_coverages.get(0).unwrap().filename);
        assert_eq!(21, coverage.test_coverages.get(0).unwrap().file_coverages.get(0).unwrap().covered_lines.len());
    }
}
