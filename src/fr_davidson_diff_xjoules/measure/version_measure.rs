use serde_derive::{Deserialize, Serialize};

use super::test_measure::TestMeasure;

#[derive(Serialize, Deserialize)]
pub struct VersionMeasure {
    pub test_measures: Vec<TestMeasure>,
}

impl VersionMeasure {
    pub fn merge(&mut self, that: VersionMeasure) {
        for test_measure in that.test_measures.into_iter() {
            match self
                .test_measures
                .iter_mut()
                .find(|test_measure_to_update| {
                    test_measure_to_update.test_identifier == test_measure.test_identifier
                }) {
                Some(test_measure_found) => {
                    test_measure_found.measures.extend(test_measure.measures)
                }
                None => self.test_measures.push(test_measure),
            }
        }
    }
    pub fn find_test_measure(&self, test_identifier: &str) -> Option<&TestMeasure> {
        return self
            .test_measures
            .iter()
            .find(|test_measure| test_measure.test_identifier.eq(test_identifier));
    }
    pub fn new() -> VersionMeasure {
        return VersionMeasure {
            test_measures: Vec::new(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fr_davidson_diff_xjoules::measure::data::Data;

    #[test]
    fn test_version_measure_find_test_measure() {
        let mut version_measure_1 = VersionMeasure::new();
        let test_measure_test_1: TestMeasure = TestMeasure {
            test_identifier: String::from("test1"),
            measures: Vec::new(),
        };
        let test_measure_test_2: TestMeasure = TestMeasure {
            test_identifier: String::from("test2"),
            measures: Vec::new(),
        };

        version_measure_1.test_measures.push(test_measure_test_1);
        version_measure_1.test_measures.push(test_measure_test_2);

        assert_eq!(
            "test1",
            version_measure_1
                .find_test_measure("test1")
                .unwrap()
                .test_identifier
        );
        assert_eq!(
            "test1",
            version_measure_1
                .find_test_measure(&String::from("test1"))
                .unwrap()
                .test_identifier
        );
        assert_eq!(
            "test2",
            version_measure_1
                .find_test_measure("test2")
                .unwrap()
                .test_identifier
        );
        assert!(version_measure_1
            .find_test_measure("does_not_exist")
            .is_none());
    }

    #[test]
    fn test_version_measure_merge() {
        let mut version_measure_1: VersionMeasure = VersionMeasure::new();
        let mut test_measure_test_1: TestMeasure = TestMeasure {
            test_identifier: String::from("test1"),
            measures: Vec::new(),
        };
        let mut data_1 = Vec::new();
        data_1.push(Data {
            indicator: String::from("instructions"),
            value: 1000.0,
        });
        data_1.push(Data {
            indicator: String::from("cycles"),
            value: 2000.0,
        });
        test_measure_test_1.measures.push(data_1);
        version_measure_1.test_measures.push(test_measure_test_1);
        let mut test_measure_test_2: TestMeasure = TestMeasure {
            test_identifier: String::from("test2"),
            measures: Vec::new(),
        };
        let mut data_2 = Vec::new();
        data_2.push(Data {
            indicator: String::from("instructions"),
            value: 1000.0,
        });
        data_2.push(Data {
            indicator: String::from("cycles"),
            value: 2000.0,
        });
        test_measure_test_2.measures.push(data_2);
        version_measure_1.test_measures.push(test_measure_test_2);

        let mut version_measure_2 = VersionMeasure::new();
        let mut test_measure_test_3: TestMeasure = TestMeasure {
            test_identifier: String::from("test1"),
            measures: Vec::new(),
        };
        let mut data_3 = Vec::new();
        data_3.push(Data {
            indicator: String::from("instructions"),
            value: 1000.0,
        });
        data_3.push(Data {
            indicator: String::from("cycles"),
            value: 2000.0,
        });
        test_measure_test_3.measures.push(data_3);
        version_measure_2.test_measures.push(test_measure_test_3);
        let mut test_measure_test_4: TestMeasure = TestMeasure {
            test_identifier: String::from("test3"),
            measures: Vec::new(),
        };
        let mut data_4 = Vec::new();
        data_4.push(Data {
            indicator: String::from("instructions"),
            value: 1000.0,
        });
        data_4.push(Data {
            indicator: String::from("cycles"),
            value: 2000.0,
        });
        test_measure_test_4.measures.push(data_4);
        version_measure_2.test_measures.push(test_measure_test_4);

        assert_eq!(2, version_measure_1.test_measures.len());
        assert_eq!(1, version_measure_1.test_measures[0].measures.len());
        assert_eq!(1, version_measure_1.test_measures[1].measures.len());

        version_measure_1.merge(version_measure_2);

        assert_eq!(3, version_measure_1.test_measures.len());
        assert_eq!(2, version_measure_1.test_measures[0].measures.len());
        assert_eq!(1, version_measure_1.test_measures[1].measures.len());
    }
}
