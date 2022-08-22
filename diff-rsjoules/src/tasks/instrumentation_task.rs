use super::Task;

pub struct InstrumentationTask {}

impl InstrumentationTask {
    pub fn new() -> InstrumentationTask {
        return InstrumentationTask {};
    }
}

#[allow(unused)]
impl Task for InstrumentationTask {
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
