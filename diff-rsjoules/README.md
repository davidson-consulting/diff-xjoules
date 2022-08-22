# Diff-RSJoules

Diff-JSJoules is a plugin to be used by Diff-XJoules for Rust projects.

It requires to install **Diff-XJoules**, from `diff-xjoules` folder, run:

```sh
cargo install --path .
```

Then, build **Diff-RSJoules**, from `diff-rsjoules` folder, run:

```sh
cargo build
```

:construction:

## Configuration file example

:construction:

## Configuration file for TODO

:construction:

## Command line usage

```
diff-rsjoules 1.0.0
Benjamin Danglot <bdanglot@gmail.com>
Rust Plugin for Diff-XJoules to measure the impact of commits on energy consumption exploiting
existing test developers

USAGE:
    diff-rsjoules --path-to-project-v1 <PATH_TO_PROJECT_V1> --path-to-project-v2 <PATH_TO_PROJECT_V2> --tests <TESTS> --output-path <OUTPUT_PATH> --path-diff-file <PATH_DIFF_FILE> <TASK>

ARGS:
    <TASK>    Task to perform [possible values: coverage, instrumentation, execution]

OPTIONS:
    -d, --path-diff-file <PATH_DIFF_FILE>            Path to the file containing the UNIX diff
    -f, --path-to-project-v1 <PATH_TO_PROJECT_V1>    Path to the program in the first version
    -h, --help                                       Print help information
    -o, --output-path <OUTPUT_PATH>                  Path to the output
    -s, --path-to-project-v2 <PATH_TO_PROJECT_V2>    Path to the program in the second version
    -t, --tests <TESTS>                              Path to the json file of tests set
    -V, --version
```