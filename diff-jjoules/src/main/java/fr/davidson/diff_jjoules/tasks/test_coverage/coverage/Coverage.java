package fr.davidson.diff_jjoules.tasks.test_coverage.coverage;

import java.util.ArrayList;
import java.util.List;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 13/06/2022
 */
public class Coverage {

    public final List<TestCoverage> test_coverages;

    public Coverage() {
        this.test_coverages = new ArrayList<>();
    }

    public Coverage(List<TestCoverage> test_coverages) {
        this.test_coverages = test_coverages;
    }

    public boolean containsTestIdentifier(String testIdentifier) {
        return this.findByTestIdentifier(testIdentifier) != null;
    }

    public TestCoverage findByTestIdentifier(String testIdentifier) {
        return this.test_coverages.stream()
                .filter(testCoverage -> testIdentifier.equals(testCoverage.test_identifier))
                .findFirst()
                .orElse(null);
    }
}