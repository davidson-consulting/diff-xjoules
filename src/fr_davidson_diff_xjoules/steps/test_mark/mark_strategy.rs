use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use self::strict_mark_strategy::StrictMarkStrategy;

pub mod strict_mark_strategy;

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
