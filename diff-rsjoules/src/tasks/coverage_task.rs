use super::Task;

pub struct CoverageTask {}

impl CoverageTask {
    pub fn new() -> CoverageTask {
        return CoverageTask {};
    }
}

#[allow(unused)]
impl Task for CoverageTask {
    fn run(
        &self,
        path_to_project_v1: &str,
        path_to_project_v2: &str,
        tests: &str,
        output_path: &str,
        path_diff_file: &str,
    ) {
        todo!();
    }
}
