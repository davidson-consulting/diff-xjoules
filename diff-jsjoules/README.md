# Diff-JSJoules

Diff-JSJoules is a plugin to be used by Diff-XJoules for JS projects using jest for the tests.

It requires `jest` and [`jscodeshift`](https://github.com/facebook/jscodeshift), accessible globally:

```sh
sudo npm install jest --global
sudo npm install jscodeshift --global
```

It also requires to have [TLPC-sensor](https://github.com/davidson-consulting/tlpc-sensor) server running.

## Configuration file example

This project provides an example of configuration files and a toy-project on which you can run `diff-xjoules` using `diff-jsjoules`.

To run on the example, use the following command line:

```
cargo run diff-jsjoules/configuration_file_example.yaml
```

You should obtain, in the `"diff-jsjoules/output_example"` folder, the following files:

```sh
...
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
$ ls diff-jsjoules/output_example 
coverage_v1.json  coverage_v2.json  data_v1.json  data_v2.json  decisions.json  delta.json  diff  test_filter_selection.json  test_selection.json
```

## Configuration file for jscodeshift

In this section, we propose to execute `diff-xjoules` using `diff-jsjoules` on [jscodeshift](https://github.com/facebook/jscodeshift).

1. Clone twice `jscodeshift`:

```sh
git clone https://github.com/facebook/jscodeshift /tmp/v1
git clone https://github.com/facebook/jscodeshift /tmp/v2
```

2. Hard reset on specific commits:

```sh
git -C /tmp/v1 reset --hard adef04b97621b19420fd67919c2764910e15dabc
git -C /tmp/v2 reset --hard 514f8c3e4e2e2801713beff93a04f8f8a771fe96
```

3. Execute `diff-xjoules` using `diff-jsjoules`:

```
cargo run diff-jsjoules/configuration_file_jscodeshift.yaml
```

4. After a while, you should observe the following:

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
$ ls diff-jsjoules/output_jscodeshift
coverage_v1.json  coverage_v2.json  data_v1.json  data_v2.json  decisions.json  delta.json  diff  test_filter_selection.json  test_selection.json
```

## Command line usage

```
diff-jsjoules.js [command]

Commands:
  diff-jsjoules.js coverage         Perform the coverage task
  diff-jsjoules.js instrumentation  Perform the instrumentation task
  diff-jsjoules.js execution        Perform the execution task

Options:
      --version             Show version number                        [boolean]
  -f, --path-to-project-v1  Path to the program in the first version.   [string]
  -s, --path-to-project-v2  Path to the program in the second version.  [string]
  -t, --tests               Path to the json file of tests set.         [string]
  -o, --output-path         Path to the output.                         [string]
  -d, --path-diff-file      Path to the file containing the UNIX diff.  [string]
  -h, --help                Show help                                  [boolean]
```