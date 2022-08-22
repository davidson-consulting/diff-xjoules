use core::fmt;

use serde_derive::{Deserialize, Serialize};

use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_xjoules_data::DiffXJoulesData,
    steps::test_selection::TestSelection,
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
    fn get(&self) -> Box<dyn MarkStrategy> {
        match self {
            MarkStrategyEnum::Strict => Box::new(StrictMarkStrategy::new()),
            MarkStrategyEnum::Aggregate => Box::new(AggregateMarkStrategy::new()),
            MarkStrategyEnum::Vote => Box::new(VoteMarkStrategy::new()),
            MarkStrategyEnum::CodeCov => Box::new(CodeCoverageMarkStrategy::new()),
            MarkStrategyEnum::DiffCov => Box::new(DiffCodeCoverageMarkStrategy::new()),
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

pub trait MarkStrategy {
    fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool;
}
