pub mod fr_davidson_diff_xjoules;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
/// Rust Plugin for Diff-XJoules to measure the impact of commits on energy consumption exploiting existing test developers
struct Cli {
    #[clap(parse(from_os_str))]
    /// Path to the YAML configuration file
    yaml_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    fr_davidson_diff_xjoules::run(args.yaml_path);
}
