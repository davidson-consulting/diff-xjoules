use crate::fr_davidson_diff_xjoules::{Data, DiffXJoulesData, TestMeasure};

pub fn run(diff_xjoules_data: &mut DiffXJoulesData) {
    for selected_test in diff_xjoules_data.test_selection.test_selection.iter() {
        let test_measure_v1 = diff_xjoules_data
            .data_v1
            .find_test_measure(&selected_test)
            .unwrap();
        let test_measure_v2 = diff_xjoules_data
            .data_v2
            .find_test_measure(&selected_test)
            .unwrap();
        let mut median_v1 = TestMeasure {
            test_identifier: String::from(selected_test),
            measures: Vec::new(),
        };
        let mut median_v2 = TestMeasure {
            test_identifier: String::from(selected_test),
            measures: Vec::new(),
        };
        let mut delta = TestMeasure {
            test_identifier: String::from(selected_test),
            measures: Vec::new(),
        };
        for indicator in test_measure_v1.get_all_indicators() {
            compute_median_and_delta_indicator(
                test_measure_v1,
                test_measure_v2,
                &mut median_v1,
                &mut median_v2,
                &mut delta,
                &indicator,
            )
        }
        diff_xjoules_data.median_v1.test_measures.push(median_v1);
        diff_xjoules_data.median_v2.test_measures.push(median_v2);
        diff_xjoules_data.delta.test_measures.push(delta);
    }
}

fn compute_median_and_delta_indicator(
    test_measure_v1: &TestMeasure,
    test_measure_v2: &TestMeasure,
    median_v1: &mut TestMeasure,
    median_v2: &mut TestMeasure,
    delta: &mut TestMeasure,
    indicator: &str,
) {
    let mut median_indicator_v1 = Vec::<Data>::new();
    median_indicator_v1.push(Data {
        indicator: String::from(indicator),
        value: test_measure_v1.get_median(&indicator),
    });
    let mut median_indicator_v2 = Vec::<Data>::new();
    median_indicator_v2.push(Data {
        indicator: String::from(indicator),
        value: test_measure_v2.get_median(&indicator),
    });
    let mut delta_indicator = Vec::<Data>::new();
    delta_indicator.push(Data {
        indicator: String::from(format!("delta_{}", indicator)),
        value: median_indicator_v2[0].value - median_indicator_v1[0].value,
    });
    median_v1.measures.push(median_indicator_v1);
    median_v2.measures.push(median_indicator_v2);
    delta.measures.push(delta_indicator);
}
