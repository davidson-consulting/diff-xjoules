use clap::{ArgEnum, Parser};

pub mod tasks;

use tasks::{
    coverage_task::CoverageTask, execution_task::ExecutionTask,
    instrumentation_task::InstrumentationTask, Task,
};

#[derive(PartialEq, Debug, Clone, ArgEnum)]
pub enum TaskEnum {
    Coverage,
    Instrumentation,
    Execution,
}

#[derive(Parser)]
#[clap(author, version, about)]
/// Rust Plugin for Diff-XJoules to measure the impact of commits on energy consumption exploiting existing test developers
struct Cli {
    #[clap(arg_enum)]
    /// Task to perform
    task: TaskEnum,
    #[clap(short = 'f', long)]
    /// Path to the program in the first version.
    path_to_project_v1: String,
    #[clap(short = 's', long)]
    /// Path to the program in the second version.
    path_to_project_v2: String,
    #[clap(short, long)]
    /// Path to the json file of tests set.
    tests: String,
    #[clap(short, long)]
    /// Path to the output.
    output_path: String,
    #[clap(short = 'd', long, required = false)]
    /// Path to the file containing the UNIX diff.
    path_diff_file: String,
}

fn main() {
    let args: Cli = Cli::parse();
    match args.task {
        TaskEnum::Coverage => {
            CoverageTask::new().run(
                &args.path_to_project_v1,
                &args.path_to_project_v2,
                &args.tests,
                &args.output_path,
                &args.path_diff_file,
            );
        }
        TaskEnum::Instrumentation => {
            InstrumentationTask::new().run(
                &args.path_to_project_v1,
                &args.path_to_project_v2,
                &args.tests,
                &args.output_path,
                &args.path_diff_file,
            );
        }
        TaskEnum::Execution => {
            ExecutionTask::new().run(
                &args.path_to_project_v1,
                &args.path_to_project_v2,
                &args.tests,
                &args.output_path,
                &args.path_diff_file,
            );
        }
    }
}
