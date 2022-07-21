# Advanced Configuration

## Test Filters

In **Diff-XJoules**, test filters are strategy to discard tests that give (too) unstable energy consumption since a unstable energy consumption might results with wrong decision in the end.

We describe in the following, the test filters implemented in **Diff-XJoules**:

- `ALL`: takes all the tests regardless the stability of their data.
- `EMPTY_INTERSECTION`: for each test, it takes the two sets of energy measurement: $v_1$ and $v_2$, then it computes the intersection of the two sets. This filter takes only the tests that the intersection is empty.
- `STUDENT_T_TEST`: uses a statistical $t$-test to determine if the measurements of the test for both versions are distinct. It also uses Cohen's D to takes into account the magnitude of the difference.

## Mark Strategies

In **Diff-XJoules**, mark strategies are used to decide whether or not the build should be marked as **breaking** or **passing**.
These strategies are based on the energy consumption of the tests filtered by the test filters explained above.

We describe in the following, the mark strategies implemented in **Diff-XJoules**:

- `STRICT`: considers a commit as **breaking** if there is at least one delta test-wise is greater than zero: $\exists\Delta(t) > 0$,
- `AGG`: considers a commit as **breaking** if the sum of the delta is greater than zero: there is at least one $(\sum^{t_n}_{t_{0}} \Delta(t)) > 0$,
- `VOTE`: considers a commit as **breaking** if the number of test that have a positive delta is greater than the number of test that have a negative delta: $nb(\Delta(t) > 0) > nb(\Delta(t) < 0)$
- `COCOV`: considers a commit as **breaking** if the sum of the delta, weighted by the code coverage of the test is greater than zero: $(\sum^{t_n}_{t_{0}} \Delta(t) \times \omega_{cocov}) > 0$,
- `DICOV`: considers a commit as **breaking** if the sum of the delta, weighted by the diff coverage of the test is greater than zero: $(\sum^{t_n}_{t_{0}} \Delta(t) \times \omega_{dicov}) > 0$.
