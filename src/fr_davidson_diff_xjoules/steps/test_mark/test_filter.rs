use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{
    steps::test_selection::TestSelection, utils::json_utils, Configuration, DiffXJoulesData,
};

#[derive(Deserialize)]
pub enum TestFilterEnum {
    ALL,
}

impl TestFilterEnum {
    pub fn get(&self) -> impl TestFilter {
        match self {
            Self::ALL => AllTestFilter::new(),
        }
    }
}

pub trait TestFilter {
    fn filter(&self, configuration: &Configuration, data: &DiffXJoulesData) -> TestSelection;
    fn report(&self, configuration: &Configuration, test_selection: &TestSelection) {
        json_utils::write_json::<TestSelection>(
            &format!(
                "{}/test_filter_selection.json",
                configuration.path_output_dir
            ),
            &test_selection,
        );
    }
}

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
