use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{
    steps::test_selection::TestSelection, utils::json_utils, Configuration, DiffXJoulesData,
};

use self::{
    all_test_filter::AllTestFilter, empty_intersection_test_filter::EmptyIntersectionTestFilter,
};

pub mod all_test_filter;
pub mod empty_intersection_test_filter;

#[derive(Deserialize)]
pub enum TestFilterEnum {
    All,
    EmptyIntersection,
}
pub enum TestFilterTypedEnum {
    All(AllTestFilter),
    EmptyIntersection(EmptyIntersectionTestFilter),
}

impl TestFilterEnum {
    pub fn get(&self) -> TestFilterTypedEnum {
        match self {
            TestFilterEnum::All => TestFilterTypedEnum::All(AllTestFilter::new()),
            TestFilterEnum::EmptyIntersection => {
                TestFilterTypedEnum::EmptyIntersection(EmptyIntersectionTestFilter::new())
            }
        }
    }
}

impl TestFilterTypedEnum {
    pub fn filter(&self, configuration: &Configuration, data: &DiffXJoulesData) -> TestSelection {
        match self {
            TestFilterTypedEnum::All(ref all_test_filter) => {
                all_test_filter.filter(configuration, data)
            }
            TestFilterTypedEnum::EmptyIntersection(ref empty_intersection_test_filter) => {
                empty_intersection_test_filter.filter(configuration, data)
            }
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
