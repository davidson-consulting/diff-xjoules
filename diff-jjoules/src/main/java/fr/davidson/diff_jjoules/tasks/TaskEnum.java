package fr.davidson.diff_jjoules.tasks;

import fr.davidson.diff_jjoules.tasks.test_coverage.TestCoverageTask;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public enum TaskEnum {
    TEST_COVERAGE() {
        @Override
        public Task getTask() {
            return new TestCoverageTask();
        }
    },
    TEST_INSTRUMENTATION() {
        @Override
        public Task getTask() {
            throw new UnsupportedOperationException();
        }
    },
    TEST_EXECUTION() {
        @Override
        public Task getTask() {
            throw new UnsupportedOperationException();
        }
    };
    public abstract Task getTask();
}
