# Diff-XJoules

**Diff-XJoules** is developped in Rust but aims to be used on any programming language.
However, some part of the process is language-specific, and therefore **Diff-XJoules** is built to call external process to support language-specific parts.

For more information, please refer to [this](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/approach.md).

## Usage

To execute **Diff-XJoules**, you need to provide the path to a configuration YAML file (described in the next section).
The command line is as follow:

```sh
cargo run -- <path_to_yaml_file>
```

### Configuration YAML file

The configuration YAML file requires the following properties: 

- `path_v1`: absolute path to the project **before** the commit
- `path_v2`: absolute path to the project **after** the commit
- `path_output_dir`: path to the output directory of diff-xjoules
- `src_folder`: relative path (from properties `path_v1` and `path_v2`) to folder that contains the sources
- `time_to_wait_in_millis`: the number of milliseconds to wait for chilling
- `iteration_warmup`: the number of iteration for the warmup
- `iteration_run`: the number of iteration for the measure
- `coverage_cmd`: the command to be executed to compute the code coverage of the tests
- `instrumentation_cmd`: the command to be executed to instrument the tests
- `execution_cmd`: the command to execute the tests
- `test_filter`: the test filter to use for the decision process. Values avalaible: `All`, `EmptyIntersection`.
- `mark_strategy`: the strategy for marking the decision. Values avalaible: `STRICT`.
- `indicator_to_consider_for_marking`: the indicator (energy, cycles, instructions, etc) to use for the decision process

Following, an example of configuration YAML file for Java (using [diff-jjoules](https://github.com/davidson-consulting/diff-xjoules/tree/main/diff-jjoules)):

```yaml
path_v1: "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project"
path_v2: "diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2"
path_output_dir: "target"
src_folder: "src/main/java"
time_to_wait_in_millis: 3000
iteration_warmup: 0
iteration_run: 3
coverage_cmd: "java -jar diff-jjoules/target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project {{ path_project }} --task TEST_COVERAGE --output-path {{ output_path }}"
instrumentation_cmd: "java -jar diff-jjoules/target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project {{ path_project }} --task TEST_INSTRUMENTATION --tests-set {{ tests_set_path }}"
execution_cmd: "java -jar diff-jjoules/target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project {{ path_project }} --task TEST_EXECUTION --tests-set {{ tests_set_path }}"
test_filter: ALL
mark_strategy: STRICT
indicator_to_consider_for_marking: "cycles"
```

### Languages-specific parts

As shown in the list of properties, there are 3 commands that the use must provide. 
These commands are language-specific.

For more information, please refer to [this](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/approach.md).

You can find dedicated documentation for each language-specific parts:

- [Coverage](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/coverage.md)
- [Instrumentation](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/instrumentation.md)
- [Execution](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/execution.md)

### Supported languages

Here, the supported languages, with the link to the plugins:

- Java: [diff-jjoules](https://github.com/davidson-consulting/diff-xjoules/tree/main/diff-jjoules)

## Contribution

If you have any questions, remarks, suggestions or bug reports, please do not hesitate to open an issue. 
**Diff-XJoules** is licensed under **TODO**. 
Contributions and pull requests are very welcome smiley. 
For more information on contributing, see the dedicated [documentation](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/contributing.md).

## Full fist of documentations:

- [Approach](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/approach.md)
- [Coverage](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/coverage.md)
- [Instrumentation](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/instrumentation.md)
- [Execution](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/execution.md)
- [Advanced Configuration](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/advanced_configuration.md)
- [Contributing](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/contributing.md)