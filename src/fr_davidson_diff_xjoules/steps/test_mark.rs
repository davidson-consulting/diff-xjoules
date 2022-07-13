use crate::fr_davidson_diff_xjoules::{Configuration, DiffXJoulesData, VersionMeasure};

use self::{mark_strategy::MarkStrategy, test_filter::TestFilter};

pub mod mark_strategy;
pub mod test_filter;

pub fn run(configuration: &Configuration, diff_xjoules_data: & mut DiffXJoulesData)  {
    let test_filter = configuration.test_filter.get();
    let test_selection = test_filter.filter(diff_xjoules_data);
    if test_selection.test_selection.is_empty() {
        
    }
    let mark_strategy = configuration.mark_strategy.get();
    let decision = mark_strategy.decide(configuration, diff_xjoules_data, &test_selection);
    diff_xjoules_data.mark_test_selection = test_selection;
    diff_xjoules_data.decision = decision;
}