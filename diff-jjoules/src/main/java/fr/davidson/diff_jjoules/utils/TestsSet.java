package fr.davidson.diff_jjoules.utils;

import java.util.HashSet;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 20/06/2022
 */
public class TestsSet {

    public final HashSet<String> testSelection;

    public TestsSet() {
        this.testSelection = new HashSet<>();
    }

    public TestsSet(HashSet<String> testSelection) {
        this.testSelection = testSelection;
    }
}
