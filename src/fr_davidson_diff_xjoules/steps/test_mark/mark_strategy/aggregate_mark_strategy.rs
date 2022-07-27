use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use super::MarkStrategy;

pub struct AggregateMarkStrategy {}

impl MarkStrategy for AggregateMarkStrategy {
    fn decide(
        &self,
        configuration: &Configuration,
        data: &DiffXJoulesData,
        test_selection: &TestSelection,
    ) -> bool {
        return test_selection
            .test_selection
            .iter()
            .map(|selected_test| {
                let delta_test = data.delta.find_test_measure(selected_test).unwrap();
                return delta_test.measures[0]
                    .iter()
                    .find(|data| {
                        data.indicator
                            .eq(&configuration.indicator_to_consider_for_marking)
                    })
                    .unwrap()
                    .value;
            })
            .sum::<f64>()
            > 0.0;
    }
}

impl AggregateMarkStrategy {
    pub fn new() -> AggregateMarkStrategy {
        AggregateMarkStrategy {}
    }
}
