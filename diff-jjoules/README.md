# Diff-JJoules

Diff-JJoules is a plugin to be used by Diff-XJoules for Java projects using Maven.

## Command line

```
java -jar target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --help
Usage: fr.davidson.diff_jjoules.Main [-hV] -o=<outputPath> -p=<pathToProject> --step=<step>
    -h, --help          Show this help message and exit.
    -o, --output-path=<outputPath>
    Path to the output.
    -p, --path-to-project=<pathToProject>
    Path to the program.
    --step=<step>   Specify the step to perform.Valid values: TEST_SELECTION, TEST_INSTRUMENTATION, TEST_EXECUTION
    -V, --version       Print version information and exit.
```