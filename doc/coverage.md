# Coverage

The **Coverage computation** should produces a json file that gives the coverage of each source file for every tests, identified by a unique string.
The **Coverage computation** is executed for each version of the program, before and after the code changes.

## Input

The process of coverage should take as input two parameters, which **Diff-XJoules** will provide according to its configuration.

- `{{ path_project }}`: the path to the root of the project. This input is replaced once by the property `path_v1` and once by the property `path_v2`.
- `{{ output_path }}`: the path to the output json file folder. This input is replaced once by the property `path_output_dir` concatenated to `coverage_v1.json`, and once concatenated to `coverage_v2.json`.

## Output

The output of the process should be a json file per version of the program that associates test identifiers (_i.e._ unique string identify a test case) to the coverage per file name.
The coverage per filename is an association between the file path from the root of the project to the line covered by the considered test case.

Below, an excerpt of an example of coverage for Java:

```json
{
  "test_coverages": [
    {
      "test_identifier": "fr.davidson.AppTest#testRandomQuickSort",
      "file_coverages": [
        {
          "filename": "src/main/java/fr/davidson/App.java",
          "covered_lines": [
            15,
            16,
            17,
            19,
            21,
            22,
            23,
            26,
            27,
            28,
            29,
            30,
            32,
            36,
            37,
            40,
            41,
            42,
            16,
            27,
            29
          ]
        },
        ...
    }]
}
```

Here, we see that the test considered is `fr.davidson.AppTest#testRandomQuickSort` which the full qualified name of the test method.
The test identifier format has no requirements, unless to be unique for each test.
It shows the coverage for one file, which is `src/main/java/fr/davidson/App.java`. 
We see that the path is relative from the root of the project.
Then, the field `covered_lines` is an array containing all the covered line in the file (_i.e._ `src/main/java/fr/davidson/App.java`) by the considered test (identified in this case by its full qualified name method, `fr.davidson.AppTest#testRandomQuickSort`).

## Command line example

Below, a command line example for Java:

```sh
java -jar diff-jjoules/target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project {{ path_project }} --task TEST_COVERAGE --output-path {{ output_path }}
```

**Diff-XJoules** will execute this command twice, once per version of the program (before and after the commit).
In this command, we can observe the two templates `{{ path_project }}` and `{{ output_path }}`.
The former will be replaced sequentially by the values of the properties `path_v1` and `path_v2`, while the latter will be replaced by the value of the property `path_output_dir` once concatenated to `coverage_v1.json`, and once concatenated to `coverage_v2.json`.

If we consider [this](https://github.com/davidson-consulting/diff-xjoules/blob/main/test_resources/configuration_file_example.yaml) configuration YAML file example, the two resulting command executed will be:

```sh
java -jar diff-jjoules/target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project diff-jjoules/src/test/resources/diff-jjoules-toy-java-project --task TEST_COVERAGE --output-path path_output_dir/coverage_v1.json
```
```sh
java -jar diff-jjoules/target/diff-jjoules-0.1.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2 --task TEST_COVERAGE --output-path path_output_dir/coverage_v2.json
```