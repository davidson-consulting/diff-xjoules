package fr.davidson.diff_jjoules.utils;

import java.util.HashSet;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 20/06/2022
 */
public class TestsSet {

    public final HashSet<String> test_selection;

    public TestsSet(HashSet<String> testSelection) {
        this.test_selection = testSelection;
    }

    public String[] getTestClassNames() {
        return test_selection.stream()
                .map(testIdentifier ->
                        new MethodFullQualifiedName(testIdentifier).getClassFullQualifiedName()

                ).distinct()
                .toArray(String[]::new);
    }

    public String[] getTestMethodNames() {
        return test_selection.stream()
                .map(testIdentifier ->
                        new MethodFullQualifiedName(testIdentifier).methodName
                ).toArray(String[]::new);
    }
}
