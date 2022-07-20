# Coverage

## Input

The process of coverage should take as input two parameters, which **Diff-XJoules** will provide according to its configuration.

- `path_project`: the path to the root of the project.
- `output_path`: the path to the ouput folder.

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
It shows the coverage for one file, which is `src/main/java/fr/davidson/App.java`. We see that the path is relative from the root of the project.
Then, the field `covered_lines` is an array containing all the covered line in the file (_i.e._ `src/main/java/fr/davidson/App.java`) by the considered test (identified in this case by its full qualified name method, `fr.davidson.AppTest#testRandomQuickSort`).