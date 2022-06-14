use serde_derive::Deserialize;
use handlebars::Handlebars;

use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::utils::{command, json_utils};

pub const COVERAGE_FILENAME: &str = "coverage";

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

pub fn run_coverage_cmd(path_to_project: String, coverage_cmd: String, output_path: String) -> Coverage {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("coverage_cmd", coverage_cmd)
        .unwrap();
    let mut data = HashMap::new();
    data.insert("path_project", path_to_project);
    data.insert("output_path", output_path.clone());
    command::run_command(handlebars.render("coverage_cmd", &data).unwrap());
    return json_utils::read_json::<Coverage>(output_path.clone());
}