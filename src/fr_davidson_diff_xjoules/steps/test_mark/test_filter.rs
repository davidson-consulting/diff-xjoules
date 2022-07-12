use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{DiffXJoulesData, steps::test_selection::TestSelection};

#[derive(Deserialize)]
pub enum TestFilterEnum {
    ALL
}

impl TestFilterEnum {
    pub fn get(&self) -> impl TestFilter {
        match self {
            Self::ALL => AllTestFilter::new(),
        }
    }
}

pub trait TestFilter {
    fn filter(self, data: &DiffXJoulesData) -> TestSelection;
}

pub struct AllTestFilter {

}

impl TestFilter for AllTestFilter {
    fn filter(self, data: &DiffXJoulesData) -> TestSelection {
        let mut test_selection = TestSelection::new();
        for selected_test in data.test_selection.test_selection.iter() {
            test_selection.test_selection.insert(selected_test.to_string());
        }
        return test_selection;
    }
}

impl AllTestFilter {
    pub fn new() -> AllTestFilter {
        AllTestFilter { }
    }
}