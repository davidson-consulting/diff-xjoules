pub mod fr_davidson_diff_xjoules;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    yaml_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    fr_davidson_diff_xjoules::run(args.yaml_path);
}