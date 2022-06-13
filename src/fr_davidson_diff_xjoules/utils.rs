use serde_derive::Deserialize;

pub mod files_utils;

#[derive(Deserialize)]
pub struct Coverage {
    pub test_coverages: Vec<TestCoverage>
}

#[derive(Deserialize)]
pub struct TestCoverage {
    pub test_identifier: String,
    pub file_coverages: Vec<FileCoverage>
}

#[derive(Deserialize)]
pub struct FileCoverage {
    pub filename: String,
    pub covered_lines: Vec<i16>
}