use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{
    steps::test_selection::TestSelection, Configuration, DiffXJoulesData,
};

#[derive(Deserialize)]
pub enum MarkStrategyEnum {
    STRICT,
}

impl MarkStrategyEnum {
    pub fn get(&self) -> impl MarkStrategy {
        match self {
            Self::STRICT => StrictMarkStrategy::new(),
        }
    }
}

pub trait MarkStrategy {
    fn decide(
        self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool;
}

pub struct StrictMarkStrategy {}

// For now, we base all our MarkStrategy on cycles, but we could give as input which indicator is to be taken into account
impl MarkStrategy for StrictMarkStrategy {
    fn decide(
        self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool {
        for selected_test in test_selection.test_selection.iter() {
            let delta_test = data.delta.find_test_measure(&selected_test).unwrap();
            let considered_delta = delta_test.measures[0]
                .iter()
                .find(|data| data.indicator == configuration.indicator_to_consider_for_marking)
                .unwrap();
            if considered_delta.value > 0.0 {
                return false;
            }
        }
        return true;
    }
}

impl StrictMarkStrategy {
    pub fn new() -> StrictMarkStrategy {
        StrictMarkStrategy {}
    }
}
