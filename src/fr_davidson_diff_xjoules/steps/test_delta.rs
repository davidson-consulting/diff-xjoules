use crate::fr_davidson_diff_xjoules::{
    utils::json_utils, Configuration, Data, DiffXJoulesData, TestMeasure, VersionMeasure,
};

pub fn run(configuration: &Configuration, diff_xjoules_data: &mut DiffXJoulesData) {
    for selected_test in diff_xjoules_data.test_selection.test_selection.iter() {
        let test_measure_v1 = diff_xjoules_data
            .data_v1
            .find_test_measure(&selected_test)
            .unwrap();
        let test_measure_v2 = diff_xjoules_data
            .data_v2
            .find_test_measure(&selected_test)
            .unwrap();
        let mut medians_v1 = Vec::<Data>::new();
        let mut medians_v2 = Vec::<Data>::new();
        let mut deltas = Vec::<Data>::new();
        for indicator in test_measure_v1.get_all_indicators() {
            compute_median_and_delta_indicator(
                &mut medians_v1,
                &mut medians_v2,
                &mut deltas,
                test_measure_v1,
                test_measure_v2,
                &indicator,
            )
        }
        let mut test_median_v1 = TestMeasure {
            test_identifier: String::from(selected_test),
            measures: Vec::new(),
        };
        let mut test_median_v2 = TestMeasure {
            test_identifier: String::from(selected_test),
            measures: Vec::new(),
        };
        let mut test_delta = TestMeasure {
            test_identifier: String::from(selected_test),
            measures: Vec::new(),
        };
        test_median_v1.measures.push(medians_v1);
        test_median_v2.measures.push(medians_v2);
        test_delta.measures.push(deltas);
        diff_xjoules_data.median_v1.test_measures.push(test_median_v1);
        diff_xjoules_data.median_v2.test_measures.push(test_median_v2);
        diff_xjoules_data.delta.test_measures.push(test_delta);
        json_utils::write_json::<VersionMeasure>(
            &format!("{}/delta.json", configuration.path_output_dir),
            &diff_xjoules_data.delta,
        );
    }
}

fn compute_median_and_delta_indicator(
    medians_v1: &mut Vec<Data>,
    medians_v2: &mut Vec<Data>,
    deltas: &mut Vec<Data>,
    test_measure_v1: &TestMeasure,
    test_measure_v2: &TestMeasure,
    indicator: &str,
) {
    let median_v1 = test_measure_v1.get_median(&indicator);
    medians_v1.push(Data {
        indicator: String::from(indicator),
        value: median_v1,
    });
    let median_v2 = test_measure_v2.get_median(&indicator);
    medians_v2.push(Data {
        indicator: String::from(indicator),
        value: median_v2,
    });
    deltas.push(Data {
        indicator: String::from(indicator),
        value: median_v2 - median_v1,
    });
}
