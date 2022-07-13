use serde_derive::{Serialize, Deserialize};

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
    pub fn get_median(&self, indicator: &str) -> f64 {
        let mut values: Vec<f64> = self
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
        return math::median(&mut values);
    }
}



mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::utils::json_utils;

    #[test]
    fn test_test_measure_get_median() {
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

        assert_eq!(20.0, test_measure_test_1.get_median("instructions"));
        assert_eq!(2000.0, test_measure_test_1.get_median("cycles"));
    }
}
