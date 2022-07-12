use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{DiffXJoulesData, steps::test_selection::TestSelection, Configuration};

#[derive(Deserialize)]
pub enum MarkStrategyEnum {
    STRICT
}

impl MarkStrategyEnum {
    pub fn get(&self) -> impl MarkStrategy {
        match self {
            Self::STRICT => StrictMarkStrategy::new(),
        }
    }
}


pub trait MarkStrategy {
    fn decide(self, configuration: &Configuration, data: &DiffXJoulesData, test_selection: &TestSelection) -> bool;
}

pub struct StrictMarkStrategy {

}

impl MarkStrategy for StrictMarkStrategy {
    fn decide(self, configuration: &Configuration, data: &DiffXJoulesData, test_selection: &TestSelection) -> bool {
        return true;
    }
}

impl StrictMarkStrategy {
    pub fn new() -> StrictMarkStrategy {
        StrictMarkStrategy { }
    }
}