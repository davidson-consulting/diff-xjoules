use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{
    steps::test_selection::TestSelection, utils::json_utils, Configuration, DiffXJoulesData,
};

use self::all_test_filter::AllTestFilter;

pub mod all_test_filter;

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