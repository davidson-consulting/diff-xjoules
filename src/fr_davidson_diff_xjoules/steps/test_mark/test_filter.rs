use core::fmt;

use serde_derive::{Deserialize, Serialize};

use crate::fr_davidson_diff_xjoules::{
    steps::test_selection::TestSelection, utils::json_utils, Configuration, DiffXJoulesData,
};

use self::{
    all_test_filter::AllTestFilter, empty_intersection_test_filter::EmptyIntersectionTestFilter,
    student_t_test_test_filter::StudentTTestTestFilter,
};

pub mod all_test_filter;
pub mod empty_intersection_test_filter;
pub mod student_t_test_test_filter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TestFilterEnum {
    All,
    EmptyIntersection,
    StudentTTest,
}
impl fmt::Display for TestFilterEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestFilterEnum::All => write!(f, "all_test"),
            TestFilterEnum::EmptyIntersection => write!(f, "empty_intersection"),
            TestFilterEnum::StudentTTest => write!(f, "student_t_test"),
        }
    }
}

impl TestFilterEnum {
    fn get(&self) -> Box<dyn TestFilter> {
        return match self {
            TestFilterEnum::All => Box::new(AllTestFilter::new()),
            TestFilterEnum::EmptyIntersection => Box::new(EmptyIntersectionTestFilter::new()),
            TestFilterEnum::StudentTTest => Box::new(StudentTTestTestFilter::new()),
        };
    }
    pub fn filter(&self, configuration: &Configuration, data: &DiffXJoulesData) -> TestSelection {
        return self.get().filter(configuration, data);
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
