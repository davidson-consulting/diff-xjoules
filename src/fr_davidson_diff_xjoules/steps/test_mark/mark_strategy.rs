use core::fmt;

use serde_derive::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl fmt::Display for MarkStrategyEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarkStrategyEnum::Strict => write!(f, "strict"),
            MarkStrategyEnum::Aggregate => write!(f, "aggregate"),
            MarkStrategyEnum::Vote => write!(f, "vote"),
            MarkStrategyEnum::CodeCov => write!(f, "code_coverage"),
            MarkStrategyEnum::DiffCov => write!(f, "diff_coverage"),
        }
    }
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
