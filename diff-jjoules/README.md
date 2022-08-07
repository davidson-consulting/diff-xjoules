# Diff-JJoules

Diff-JJoules is a plugin to be used by Diff-XJoules for Java projects using Maven.

It requires Java 11 (or more).

It also requires to install [TLPC-sensor](https://github.com/davidson-consulting/tlpc-sensor), both C library and JNI project under `examples/tlpc-sensor`.

To build, run:

```
mvn package -f diff-joules/pom.xml
```

## Configuration file example

This project provides an example of configuration files and a toy-project on which you can run `diff-xjoules` using `diff-jjoules`.

To run on the example, use the following command line:

```
cargo run diff-jjoules/configuration_file_example.yaml
```

You should obtain, in the `"diff-jjoules/output_example"` folder, the following files:

```sh
...
Applying all_test to keep stable tests
Deciding using strict
Deciding using aggregate
Deciding using vote
Deciding using code_coverage
Deciding using diff_coverage
Applying empty_intersection to keep stable tests
$ ls "diff-jjoules/output_example"/
coverage_v1.json  coverage_v2.json  data_v1.json  data_v2.json  decisions.json  delta.json  diff  test_filter_selection.json  test_selection.json
```

## Configuration file for google/gson example

In this section, we propose to execute `diff-xjoules` using `diff-jjoules` on [google/gson](https://github.com/google/gson.git).

1. Clone twice `google/gson`:

```sh
git clone https://github.com/google/gson.git /tmp/v1
git clone https://github.com/google/gson.git /tmp/v2
```

2. Hard reset on specific commits:

```sh
16_03be835_69173b0
git -C /tmp/v1 reset --hard d26c8189182fa96691cc8e0d0f312469ee0627bb
git -C /tmp/v2 reset --hard 364de8061173b4b91f4477a55059f68e765fc3d1
```

3. In order to make the code coverage working, you need to remove the `module-info.java` file in both source trees:

```sh
rm -f /tmp/v1/gson/src/main/java/module-info.java
rm -f /tmp/v2/gson/src/main/java/module-info.java
```

4. Execute `diff-xjoules` using `diff-jjoules`:

```
cargo run diff-jjoules/gson.yaml
```

5. After a while, you should observe the following:

```sh
Applying all_test to keep stable tests
Deciding using strict
Deciding using aggregate
Deciding using vote
Deciding using code_coverage
Deciding using diff_coverage
Applying empty_intersection to keep stable tests
Deciding using strict
Deciding using aggregate
Deciding using vote
Deciding using code_coverage
Deciding using diff_coverage
$ ls diff-jjoules/output_gson
coverage_v1.json  coverage_v2.json  data_v1.json  data_v2.json  decisions.json  delta.json  diff  test_filter_selection.json  test_selection.json
```

## Command line usage

```
java -jar target/diff-jjoules-1.0.0-SNAPSHOT-jar-with-dependencies.jar --help
Usage: fr.davidson.diff_jjoules.Main [-hV] -o=<outputPath> -p=<pathToProject> --step=<step>
    -h, --help          Show this help message and exit.
    -o, --output-path=<outputPath>
    Path to the output.
    -p, --path-to-project=<pathToProject>
    Path to the program.
    --step=<step>   Specify the step to perform.Valid values: TEST_SELECTION, TEST_INSTRUMENTATION, TEST_EXECUTION
    -V, --version       Print version information and exit.
```