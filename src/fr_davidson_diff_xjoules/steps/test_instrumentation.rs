use std::collections::HashMap;

use crate::fr_davidson_diff_xjoules::{
    utils::{command, json_utils::JSON_EXTENSION},
    Configuration,
};

use super::test_selection::TEST_SELECTION_FILENAME;

pub fn run(configuration: &Configuration) {
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("path_project", &configuration.path_v1);
    let tests_set_path = format!(
        "{}/{}{}",
        configuration.path_output_dir, TEST_SELECTION_FILENAME, JSON_EXTENSION
    );
    data.insert("tests_set_path", &tests_set_path);
    command::run_templated_command(&configuration.instrumentation_cmd, &data);
    data.insert("path_project", &configuration.path_v2);
    command::run_templated_command(&configuration.instrumentation_cmd, &data);
}

mod tests {
    #[test]
    fn run() {
        // assert_eq!(0, 1);
    }
}
