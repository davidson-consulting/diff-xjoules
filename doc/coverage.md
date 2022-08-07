# Coverage

The **Coverage computation** should produces a json file that gives the coverage of each source file for every tests, identified by a unique string.
The **Coverage computation** is executed for both versions of the program, before and after the code changes, in a single run.

## Input

The process of coverage can take as input four parameters, which **Diff-XJoules** will provide according to its configuration and its running state.

- `{{ path_project }}`: the path to the root of the project in **the version before** the commit. This input is replaced by the property `path_v1`.
- `{{ second_path_project }}`: the path to the root of the project in **the version after** the commit. This input is replaced by the property `path_v2`.
- `{{ path_diff_file }}`: the path to the file that contains the diff, obtained running UNIX command: `diff -r`. This file is automatically computed by **Diff-XJoules**.
- `{{ output_path }}`: the path to the output json file folder. This input is replaced once by the property `path_output_dir`.

## Output

The output of the process should be two json files, one per version of the program, that associate test identifiers (_i.e._ unique string identify a test case) to the coverage per file name.
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
java -jar diff-jjoules/target/diff-jjoules-1.0.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project {{ path_project }} --second-path-to-project {{ second_path_project }} --task TEST_COVERAGE --output-path {{ output_path }}
```

**Diff-XJoules** will execute this command once.
In this command, we can observe the three templates `{{ path_project }}`, `{{ second_path_project }}`, and `{{ output_path }}`.
- `{{ path_project }}`: will be replaced by the value of the property `path_v1`.
- `{{ second_path_project }}`: will be replaced by the value of the property `path_v2`.
- `{{ output_path }}`: will be replaced by the value of the property `path_output_dir`.

Notice that the command line for Java does not use the input `{{ path_diff_file }}` which is not necessary.

If we consider [this](https://github.com/davidson-consulting/diff-xjoules/blob/main/test_resources/configuration_file_example.yaml) configuration YAML file example, the resulting command executed will be:

```sh
java -jar diff-jjoules/target/diff-jjoules-1.0.0-SNAPSHOT-jar-with-dependencies.jar --path-to-project diff-jjoules/src/test/resources/diff-jjoules-toy-java-project --second-path-to-project diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2 --task TEST_COVERAGE --output-path path_output_dir/coverage_v1.json
```