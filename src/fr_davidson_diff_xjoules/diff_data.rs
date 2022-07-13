use super::{utils::coverage::Coverage, steps::test_selection::TestSelection, measure::VersionMeasure};

pub struct DiffXJoulesData {
    pub coverage_v1: Coverage,
    pub coverage_v2: Coverage,
    pub diff: String,
    pub test_selection: TestSelection,
    pub data_v1: VersionMeasure,
    pub data_v2: VersionMeasure,
    pub median_v1: VersionMeasure,
    pub median_v2: VersionMeasure,
    pub delta: VersionMeasure,
    pub mark_test_selection: TestSelection,
    pub decision: bool,
}

impl DiffXJoulesData {
    pub fn new() -> DiffXJoulesData {
        return DiffXJoulesData {
            coverage_v1: Coverage { test_coverages: Vec::new() },
            coverage_v2: Coverage { test_coverages: Vec::new() },
            diff: String::from(""),
            test_selection: TestSelection::new(),
            data_v1: VersionMeasure { test_measures: Vec::new() },
            data_v2: VersionMeasure { test_measures: Vec::new() },
            median_v1: VersionMeasure { test_measures: Vec::new() },
            median_v2: VersionMeasure { test_measures: Vec::new() },
            delta: VersionMeasure { test_measures: Vec::new() },
            mark_test_selection: TestSelection::new(),
            decision: false,
        };
    }
}