package fr.davidson.diff_jjoules.steps;

import fr.davidson.diff_jjoules.steps.selection.TestSelectionStep;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public enum StepEnum {
    TEST_SELECTION() {
        @Override
        public Step getStep() {
            return new TestSelectionStep();
        }
    },
    TEST_INSTRUMENTATION() {
        @Override
        public Step getStep() {
            throw new UnsupportedOperationException();
        }
    },
    TEST_EXECUTION() {
        @Override
        public Step getStep() {
            throw new UnsupportedOperationException();
        }
    };
    public abstract Step getStep();
}
