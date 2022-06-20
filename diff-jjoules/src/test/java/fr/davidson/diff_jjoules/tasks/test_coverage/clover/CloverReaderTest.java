package fr.davidson.diff_jjoules.tasks.test_coverage.clover;

import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.Coverage;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;


/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class CloverReaderTest {

    @Test
    void test() {
        // The keys used for the filename are a bit weird because we load from src/test/resources and not from the root path of the project
        final Coverage coverage = new CloverReader().read("src/test/resources/diff-jjoules-toy-java-project/src/test/resources/");
        assertTrue(coverage.containsTestIdentifier("fr.davidson.AppTest#testRandom"));
        assertTrue(coverage.findByTestIdentifier("fr.davidson.AppTest#testRandom").containsFileCoverage("/diff-jjoules-toy-java-project/src/main/java/fr/davidson/App.java"));
        final Integer[] expectedCoveredLines = new Integer[]{10, 11};
        assertEquals(expectedCoveredLines.length,
                coverage.findByTestIdentifier("fr.davidson.AppTest#testRandom")
                        .findFileCoverageByFilename("/diff-jjoules-toy-java-project/src/main/java/fr/davidson/App.java")
                        .covered_lines.size());
        for (Integer expectedCoveredLine : expectedCoveredLines) {
            assertTrue(
                    coverage.findByTestIdentifier("fr.davidson.AppTest#testRandom")
                            .findFileCoverageByFilename("/diff-jjoules-toy-java-project/src/main/java/fr/davidson/App.java")
                            .covered_lines
                            .contains(expectedCoveredLine)
            );
        }

    }
}
