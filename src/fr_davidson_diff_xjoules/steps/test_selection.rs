use std::collections::HashMap;

use handlebars::Handlebars;

use crate::fr_davidson_diff_xjoules::{Configuration, utils::{command, json_utils, coverage::Coverage}, DiffXJoulesData};

pub fn run(configuration: Configuration, mut diff_xjoules_data: DiffXJoulesData) {
    // 1 compute the coverages
    diff_xjoules_data.coverage_v1 = Some(run_coverage_for_version(
        configuration.path_v1, 
        configuration.cmd_coverage.clone(),
        configuration.path_output_coverage.clone() + "_v1.json"
    ));
    diff_xjoules_data.coverage_v2 = Some(run_coverage_for_version(
        configuration.path_v2,
         configuration.cmd_coverage.clone(),
         configuration.path_output_coverage.clone() + "_v2.json"
        ));
    // 2 run algo on diff
    // 3 output the selected tests in a JSON (and init a field / struct somewhere)
}

fn run_coverage_for_version(path_to_project: String, coverage_cmd: String, output_path: String) -> Coverage {
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