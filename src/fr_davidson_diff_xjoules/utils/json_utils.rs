use std::fs;
use serde::{de::DeserializeOwned, Serialize};

const JSON_EXTENSION: &str = ".json";

pub fn write_json<T: Serialize>(path_to_json_file: &str, data: T) {
    let json_file_content = serde_json::to_string(&data).unwrap();
    fs::write(format!("{}{}", path_to_json_file, JSON_EXTENSION), json_file_content).unwrap();
}

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

    #[test]
    fn test_write_json() {
        let mut coverage: Coverage = read_json::<Coverage>("test_resources/coverage");
        write_json::<Coverage>("target/copy_coverage", coverage);
        coverage = read_json::<Coverage>("target/copy_coverage");
        assert_eq!("fr.davidson.AppTest#testRandomQuickSort", coverage.test_coverages.get(0).unwrap().test_identifier);
        assert_eq!("src/main/java/fr/davidson/App.java", coverage.test_coverages.get(0).unwrap().file_coverages.get(0).unwrap().filename);
        assert_eq!(21, coverage.test_coverages.get(0).unwrap().file_coverages.get(0).unwrap().covered_lines.len());
    }
}
