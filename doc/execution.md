# Execution

The **Execution of Tests** should executes the tests that have been instrumented at the [previous step](https://github.com/davidson-consulting/diff-xjoules/tree/main/doc/instrumentation.md) to measure their energy consumption.
The **Execution of Tests** is executed for both versions of the program, before and after the code changes.

## Input

The process of execution should take as input three parameters, which **Diff-XJoules** will provide according to its configuration.

- `{{ path_project_v1 }}`: the path to the root of the project in **the version before** the commit. This input is replaced by the property `path_v1`.
- `{{ path_project_v2 }}`: the path to the root of the project in **the version after** the commit. This input is replaced by the property `path_v2`.
- `{{ tests_set_path }}`: the path to a json files contains the test identifiers to be instrumented. This json file is produces by **Diff-XJoules** and has the following structures:

```json
{
  "test_selection": [
    "test_identifiers_1",
    "test_identifiers_2",
    ...
}
```

## Ouput

The output of the process should be a json file that follow the scheme below: 

```json
{
  "test_measures": [
    {
      "test_identifier": "fr.davidson.AppTest#testAddedStatement",
      "measures": [
        [
          {
            "indicator": "duration",
            "value": 99231.0
          },
          {
            "indicator": "UNHALTED_REFERENCE_CYCLES",
            "value": 177375.0
          },
          {
            "indicator": "RAPL_ENERGY_PKG",
            "value": 4137.0
          },
          {
            "indicator": "INSTRUCTIONS_RETIRED",
            "value": 59071.0
          }
        ]
      ]
    },
    {
      "test_identifier": "fr.davidson.AppTest#testRemovedStatement",
      "measures": [
        ...
      ]
    }
}
```

Only the structure must be respected.
The test identifiers must be the same than used in the test code coverage and the test selection.
The `measures` field is an array of array of `measure`.
The first dimension is the runs, _i.e._ the number of time we execute the test;
The second dimension is the various measures done during each execution.
The `measures`, which are elements of the array of the same name, can be arbitrary.

On can imagine the following measure:
```json
{
    "indicator": "tacos",
    "value": 23.0
}
```

However, be aware that the indicator will be used for the mark decision thanks to the property `indicator_to_consider_for_marking`.


## Command line example

Below, a command line example for Java:

```sh
java -jar diff-jjoules/target/diff-jjoules-1.1.0-jar-with-dependencies.jar --path-to-project-v1 {{ path_project_v1 }} --path-to-project-v2 {{ path_project_v2 }} --task TEST_EXECUTION --tests-set {{ tests_set_path }}
```

**Diff-XJoules** will execute this command once for both versions of the program (before and after the commit).
In this command, we can observe the three templates `{{ path_project_v1 }}`, `{{ path_project_v2 }}`, and `{{ tests_set_path }}`.
- `{{ path_project_v1 }}`: will be replaced by the value of the property `path_v1`.
- `{{ path_project_v2 }}`: will be replaced by the value of the property `path_v2`.
- `{{ tests_set_path }}`: will be replaced by the path to the json computed by **Diff-XJoules**, which is the concatenation of the property `{{ output_path }}` and `test_selection.json`.

If we consider [this](https://github.com/davidson-consulting/diff-xjoules/blob/main/test_resources/configuration_file_example.yaml) configuration YAML file example, the resulting command executed will be:

```sh
java -jar diff-jjoules/target/diff-jjoules-1.1.0-jar-with-dependencies.jar --path-to-project-v1 diff-jjoules/src/test/resources/diff-jjoules-toy-java-project --path-to-project-v2 diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2 --task TEST_EXECUTION --tests-set target/test_selection.json
```