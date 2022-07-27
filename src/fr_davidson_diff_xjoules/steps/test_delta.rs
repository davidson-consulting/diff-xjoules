use crate::fr_davidson_diff_xjoules::{
    measure::{data::Data, test_measure::TestMeasure, version_measure::VersionMeasure},
    utils::json_utils,
    Configuration, DiffXJoulesData,
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
        diff_xjoules_data
            .median_v1
            .test_measures
            .push(test_median_v1);
        diff_xjoules_data
            .median_v2
            .test_measures
            .push(test_median_v2);
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

#[cfg(test)]
mod test {
    use crate::fr_davidson_diff_xjoules::{
        configuration::Configuration,
        diff_data::DiffXJoulesData,
        steps::{
            test_mark::{mark_strategy::MarkStrategyEnum, test_filter::TestFilterEnum},
            test_selection::TestSelection,
        },
    };

    use super::*;

    #[test]
    fn test_compute_median_and_delta_indicator() {
        let mut medians_v1 = Vec::<Data>::new();
        let mut medians_v2 = Vec::<Data>::new();
        let mut deltas = Vec::<Data>::new();
        let data_v1 = json_utils::read_json::<VersionMeasure>("test_resources/data_v1.json");
        let data_v2 = json_utils::read_json::<VersionMeasure>("test_resources/data_v2.json");
        let test_measure_v1 = data_v1
            .find_test_measure("fr.davidson.AppTest#testAddedStatement")
            .unwrap();
        let test_measure_v2 = data_v2
            .find_test_measure("fr.davidson.AppTest#testAddedStatement")
            .unwrap();
        compute_median_and_delta_indicator(
            &mut medians_v1,
            &mut medians_v2,
            &mut deltas,
            test_measure_v1,
            test_measure_v2,
            "duration",
        );
        compute_median_and_delta_indicator(
            &mut medians_v1,
            &mut medians_v2,
            &mut deltas,
            test_measure_v1,
            test_measure_v2,
            "RAPL_ENERGY_PKG",
        );
        assert_eq!(136359.0, medians_v1.get(0).unwrap().value);
        assert_eq!(142499.0, medians_v2.get(0).unwrap().value);
        assert_eq!(6140.0, deltas.get(0).unwrap().value);
        assert_eq!("duration", medians_v1.get(0).unwrap().indicator);
        assert_eq!("duration", medians_v2.get(0).unwrap().indicator);
        assert_eq!("duration", deltas.get(0).unwrap().indicator);
        assert_eq!(3580.0, medians_v1.get(1).unwrap().value);
        assert_eq!(4231.0, medians_v2.get(1).unwrap().value);
        assert_eq!(651.0, deltas.get(1).unwrap().value);
        assert_eq!("RAPL_ENERGY_PKG", medians_v1.get(1).unwrap().indicator);
        assert_eq!("RAPL_ENERGY_PKG", medians_v2.get(1).unwrap().indicator);
        assert_eq!("RAPL_ENERGY_PKG", deltas.get(1).unwrap().indicator);
    }

    #[test]
    fn test_run() {
        let configuration = Configuration {
            path_v1: String::from("diff-jjoules/src/test/resources/diff-jjoules-toy-java-project"),
            path_v2: String::from(
                "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2",
            ),
            src_folder: String::from("src/main/java"),
            path_output_dir: String::from("target"),
            coverage_cmd: String::from(""),
            instrumentation_cmd: String::from(""),
            execution_cmd: String::from(""),
            iteration_warmup: 1,
            iteration_run: 3,
            time_to_wait_in_millis: 500,
            test_filter: TestFilterEnum::All,
            mark_strategy: MarkStrategyEnum::Strict,
            indicator_to_consider_for_marking: String::from("cycles"),
        };
        let mut diff_xjoules_data = DiffXJoulesData::new();
        diff_xjoules_data.test_selection =
            json_utils::read_json::<TestSelection>("test_resources/test_filter_selection.json");
        diff_xjoules_data.data_v1 =
            json_utils::read_json::<VersionMeasure>("test_resources/data_v1.json");
        diff_xjoules_data.data_v2 =
            json_utils::read_json::<VersionMeasure>("test_resources/data_v2.json");
        assert_eq!(0, diff_xjoules_data.delta.test_measures.len());
        assert_eq!(0, diff_xjoules_data.median_v1.test_measures.len());
        assert_eq!(0, diff_xjoules_data.median_v2.test_measures.len());
        run(&configuration, &mut diff_xjoules_data);
        assert_eq!(4, diff_xjoules_data.delta.test_measures.len());
        assert_eq!(4, diff_xjoules_data.median_v1.test_measures.len());
        assert_eq!(4, diff_xjoules_data.median_v2.test_measures.len());
    }
}
