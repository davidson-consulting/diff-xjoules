use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use self::{
    aggregate_mark_strategy::AggregateMarkStrategy, strict_mark_strategy::StrictMarkStrategy,
};

pub mod aggregate_mark_strategy;
pub mod strict_mark_strategy;

#[derive(Deserialize)]
pub enum MarkStrategyEnum {
    Strict,
    Aggregate,
}

pub enum MarkStrategyTypedEnum {
    Strict(StrictMarkStrategy),
    Aggregate(AggregateMarkStrategy),
}

impl MarkStrategyEnum {
    fn get(&self) -> MarkStrategyTypedEnum {
        match self {
            MarkStrategyEnum::Strict => MarkStrategyTypedEnum::Strict(StrictMarkStrategy::new()),
            MarkStrategyEnum::Aggregate => {
                MarkStrategyTypedEnum::Aggregate(AggregateMarkStrategy::new())
            }
        }
    }
    pub fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool {
        return self.get().decide(configuration, data, test_selection);
    }
}

impl MarkStrategyTypedEnum {
    fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool {
        match self {
            MarkStrategyTypedEnum::Strict(ref strict_mark_strategy) => {
                strict_mark_strategy.decide(configuration, data, test_selection)
            }
            MarkStrategyTypedEnum::Aggregate(ref aggregate_mark_strategy) => {
                aggregate_mark_strategy.decide(configuration, data, test_selection)
            }
        }
    }
}

pub trait MarkStrategy {
    fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool;
}
