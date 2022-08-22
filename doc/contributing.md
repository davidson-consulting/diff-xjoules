# Contributing

For contributing, we favor the creation of pull request (PR) on GitHub.

## Guidelines for all pull-requests

* The pull request does a single thing (_e.g._ a single bug fix or a single feature). Simpler is better.
* The pull request must pass all continuous integration checks.
* The pull request must have an explicit and clear explanation.
* Whenever the pull request resolves an issue, add "fix #issueNumber" or "close #issueNumber" to the description.
* The pull-request title starts with a prefix stating its kind: `fix`, `test`, `feat`, `refactor`, `doc`, `ci`, or `chore`, followed by the impacted component: `diff-xjoules`, `diff-jjoules` between parenthesis. For example: `feat(xjoules): add command line option to provide confguration yaml file path` describing a new feature, on the component diff-xjoules followed by a text summarizing the new feature.
* The pull request must contain at least on test case specify the modification. 
  For example, for a bug fix, the pull request must contain a test case that was failing before the fix but passing after.     
* **Your contribution is highly welcome**! If you have anything interesting, then we welcome your PR even if it is not perfect at the beginning.

## Guide on prefixes

- `fix`: the commits intend to make a failing test to pass.
- `test`: the commits intend to improve the test.
- `feat`: the commits intend to add functionality(ies).
- `refactor`: the commits intend to modify the code without modifying the overall behavior.
- `doc`: the commits intend to modify the documentation without modifying the code.
- `ci`: the commits intend to improve or modify the continuous integration workflows.
- `chore`: the commits intend to modify others files that is nor code nor documentation, _e.g._ configuration files.

## Generating changelog

To generate a fancy and well structured Changelog in Markdown format, you can use the `changelog-generator`.

To do so, run the following command line from `diff-xjoules` root folder:

```sh
node changelog-generator/changelog.js davidson-consulting diff-xjoules 0.0.0
```

where `0.0.0` should be the previous tags.

This command will output a changelog from the provided tag to the current version.
It might need some manual modifications for the classification, authors name, etc.

Please, refer to the dedicated [README.md](https://github.com/davidson-consulting/changelog-generator/README.md) for more information.

## Releasing

In this section, we describe how we release `diff-xjoules` and its components.

1. Bump the version of all component, _e.g._ [this commit](https://github.com/davidson-consulting/diff-xjoules/commit/7af2701f5aaccf8ff0c38262f4b25c1ee6b1e513)
2. Create a new tag:

```sh
git tag diff-xjoules-X.Y.Z
```

3. push the new tag:
```sh
git push diff-xjoules-X.Y.Z
```

4. Go to the [tags section](https://github.com/davidson-consulting/diff-xjoules/tags) and create a new release from the tag your created and push in 1. and 2. : `diff-xjoules-X.Y.Z`

5. generate changelog

```sh
node changelog-generator/changelog.js davidson-consulting diff-xjoules U.V.W
```

Where `U.V.W` is the previous version, latest tag before the one you had created above: _e.g._ from `1.0.0` to `1.1.0` where `U.V.W=1.0.0` and `X.Y.Z=1.1.0`.

6. Copy and paste the change log in to GitHub. Correct potential issue (redundant lines, mis-categorized commits, etc.).

7. build all executables, for example: 

* diff-xjoules:

```sh
cargo clean
cargo build --release
```

and upload `target/release/diff-xjoules`.

* diff-jjoules:

```sh
mvn clean package -DskipTests -f `diff-jjoules`
```

and upload `target/diff-jjoules-X-Y-Z-jar-with-dependencies.jar`.