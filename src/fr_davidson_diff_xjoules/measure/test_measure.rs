use serde_derive::{Deserialize, Serialize};

use crate::fr_davidson_diff_xjoules::utils::math;

use super::data::Data;

#[derive(Serialize, Deserialize)]
pub struct TestMeasure {
    pub test_identifier: String,
    pub measures: Vec<Vec<Data>>,
}

impl TestMeasure {
    pub fn get_all_indicators(&self) -> Vec<&String> {
        let mut indicators = Vec::<&String>::new();
        self.measures[0]
            .iter()
            .for_each(|data| indicators.push(&data.indicator));
        return indicators;
    }
    pub fn get_all_measures(&self, indicator: &str) -> Vec<f64> {
        return self
            .measures
            .iter()
            .map(|datas| {
                datas
                    .iter()
                    .find(|data| data.indicator == indicator)
                    .unwrap()
                    .value
            })
            .collect();
    }
    pub fn get_median(&self, indicator: &str) -> f64 {
        return math::median(&mut self.get_all_measures(indicator));
    }
    pub fn get_max(&self, indicator: &str) -> f64 {
        return math::max(&mut self.get_all_measures(indicator));
    }
    pub fn get_min(&self, indicator: &str) -> f64 {
        return math::min(&mut self.get_all_measures(indicator));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_measure_get_all_measures() {
        let test_measure_test_1 = build_test_measure_for_test();
        let measures = test_measure_test_1.get_all_measures("cycles");
        assert_eq!(3, measures.len());
    }

    #[test]
    fn test_test_measure_get_all_indicators() {
        let test_measure_test_1 = build_test_measure_for_test();
        let indicators = test_measure_test_1.get_all_indicators();
        assert_eq!(2, indicators.len());
        assert!(indicators.contains(&&String::from("cycles")));
        assert!(indicators.contains(&&String::from("instructions")));
    }

    #[test]
    fn test_test_measure_get_median() {
        let test_measure_test_1 = build_test_measure_for_test();
        assert_eq!(20.0, test_measure_test_1.get_median("instructions"));
        assert_eq!(2000.0, test_measure_test_1.get_median("cycles"));
    }

    #[test]
    fn test_test_measure_get_max() {
        let test_measure_test_1 = build_test_measure_for_test();
        assert_eq!(30.0, test_measure_test_1.get_max("instructions"));
        assert_eq!(3000.0, test_measure_test_1.get_max("cycles"));
    }

    #[test]
    fn test_test_measure_get_min() {
        let test_measure_test_1 = build_test_measure_for_test();
        assert_eq!(10.0, test_measure_test_1.get_min("instructions"));
        assert_eq!(1000.0, test_measure_test_1.get_min("cycles"));
    }

    fn build_test_measure_for_test() -> TestMeasure {
        let mut test_measure_test_1: TestMeasure = TestMeasure {
            test_identifier: String::from("test1"),
            measures: Vec::new(),
        };
        let mut data_1 = Vec::new();
        data_1.push(Data {
            indicator: String::from("instructions"),
            value: 20.0,
        });
        data_1.push(Data {
            indicator: String::from("cycles"),
            value: 2000.0,
        });
        test_measure_test_1.measures.push(data_1);

        let mut data_2 = Vec::new();
        data_2.push(Data {
            indicator: String::from("instructions"),
            value: 10.0,
        });
        data_2.push(Data {
            indicator: String::from("cycles"),
            value: 1000.0,
        });
        test_measure_test_1.measures.push(data_2);

        let mut data_3 = Vec::new();
        data_3.push(Data {
            indicator: String::from("instructions"),
            value: 30.0,
        });
        data_3.push(Data {
            indicator: String::from("cycles"),
            value: 3000.0,
        });
        test_measure_test_1.measures.push(data_3);

        return test_measure_test_1;
    }
}
