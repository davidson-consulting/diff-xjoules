pub mod coverage_task;
pub mod execution_task;
pub mod instrumentation_task;

pub trait Task {
    fn run(
        &self,
        path_to_project_v1: &str,
        path_to_project_v2: &str,
        tests: &str,
        output_path: &str,
        path_diff_file: &str,
    );
}
