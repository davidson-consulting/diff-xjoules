use crate::fr_davidson_diff_xjoules::{Configuration, SUFFIX_V1, SUFFIX_V2, utils::{coverage::{COVERAGE_FILENAME, run_coverage_cmd}}, DiffXJoulesData};

pub fn run(configuration: Configuration, mut diff_xjoules_data: DiffXJoulesData) {
    // 1 compute the coverages
    diff_xjoules_data.coverage_v1 = Some(run_coverage_cmd(
        configuration.path_v1, 
        configuration.cmd_coverage.clone(),
        configuration.path_output_coverage.clone() + "/" + COVERAGE_FILENAME + SUFFIX_V1
    ));
    diff_xjoules_data.coverage_v2 = Some(run_coverage_cmd(
        configuration.path_v2,
         configuration.cmd_coverage.clone(),
         configuration.path_output_coverage.clone() + "/" + COVERAGE_FILENAME + SUFFIX_V2
    ));
    // 2 run algo on diff
    // 3 output the selected tests in a JSON (and init a field / struct somewhere)
}