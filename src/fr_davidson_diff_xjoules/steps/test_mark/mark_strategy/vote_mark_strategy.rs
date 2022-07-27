use crate::fr_davidson_diff_xjoules::{
    configuration::Configuration, diff_data::DiffXJoulesData, steps::test_selection::TestSelection,
};

use super::MarkStrategy;

pub struct VoteMarkStrategy {}

impl MarkStrategy for VoteMarkStrategy {
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
                if delta_test.measures[0]
                    .iter()
                    .find(|data| {
                        data.indicator
                            .eq(&configuration.indicator_to_consider_for_marking)
                    })
                    .unwrap()
                    .value
                    > 0.0
                {
                    return 1;
                } else {
                    return -1;
                }
            })
            .sum::<i32>()
            < 0;
    }
}

impl VoteMarkStrategy {
    pub fn new() -> VoteMarkStrategy {
        VoteMarkStrategy {}
    }
}

