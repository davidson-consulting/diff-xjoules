use crate::fr_davidson_diff_xjoules::{configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection};

use super::TestFilter;

pub struct AllTestFilter {}

impl TestFilter for AllTestFilter {
    fn filter(&self, configuration: &Configuration, data: &DiffXJoulesData) -> TestSelection {
        let mut test_selection = TestSelection::new();
        for selected_test in data.test_selection.test_selection.iter() {
            test_selection
                .test_selection
                .insert(selected_test.to_string());
        }
        self.report(configuration, &test_selection);
        return test_selection;
    }
}

impl AllTestFilter {
    pub fn new() -> AllTestFilter {
        AllTestFilter {}
    }
}