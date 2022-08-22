use super::Task;

pub struct ExecutionTask {}

impl ExecutionTask {
    pub fn new() -> ExecutionTask {
        return ExecutionTask {};
    }
}

impl Task for ExecutionTask {
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
