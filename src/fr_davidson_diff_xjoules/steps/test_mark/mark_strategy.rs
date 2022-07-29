use serde_derive::Deserialize;

use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use self::{
    aggregate_mark_strategy::AggregateMarkStrategy,
    code_coverage_mark_strategy::CodeCoverageMarkStrategy,
    diff_coverage_mark_strategy::DiffCodeCoverageMarkStrategy,
    strict_mark_strategy::StrictMarkStrategy, vote_mark_strategy::VoteMarkStrategy,
};

pub mod aggregate_mark_strategy;
pub mod code_coverage_mark_strategy;
pub mod diff_coverage_mark_strategy;
pub mod strict_mark_strategy;
pub mod vote_mark_strategy;

#[derive(Deserialize)]
pub enum MarkStrategyEnum {
    Strict,
    Aggregate,
    Vote,
    CodeCov,
    DiffCov,
}

pub enum MarkStrategyTypedEnum {
    Strict(StrictMarkStrategy),
    Aggregate(AggregateMarkStrategy),
    Vote(VoteMarkStrategy),
    CodeCov(CodeCoverageMarkStrategy),
    DiffCov(DiffCodeCoverageMarkStrategy),
}

impl MarkStrategyEnum {
    fn get(&self) -> MarkStrategyTypedEnum {
        match self {
            MarkStrategyEnum::Strict => MarkStrategyTypedEnum::Strict(StrictMarkStrategy::new()),
            MarkStrategyEnum::Aggregate => {
                MarkStrategyTypedEnum::Aggregate(AggregateMarkStrategy::new())
            }
            MarkStrategyEnum::Vote => MarkStrategyTypedEnum::Vote(VoteMarkStrategy::new()),
            MarkStrategyEnum::CodeCov => {
                MarkStrategyTypedEnum::CodeCov(CodeCoverageMarkStrategy::new())
            }
            MarkStrategyEnum::DiffCov => {
                MarkStrategyTypedEnum::DiffCov(DiffCodeCoverageMarkStrategy::new())
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
            MarkStrategyTypedEnum::Vote(ref vote_mark_strategy) => {
                vote_mark_strategy.decide(configuration, data, test_selection)
            }
            MarkStrategyTypedEnum::CodeCov(ref code_coverage_mark_strategy) => {
                code_coverage_mark_strategy.decide(configuration, data, test_selection)
            }
            MarkStrategyTypedEnum::DiffCov(ref diff_coverage_mark_strategy) => {
                diff_coverage_mark_strategy.decide(configuration, data, test_selection)
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
