package fr.davidson.diff_jjoules.steps.selection.coverage;

import fr.davidson.diff_jjoules.steps.selection.TestSelectionStep;
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
        final TestSelectionStep.Coverage coverage = new CloverReader().read("src/test/resources/diff-jjoules-toy-java-project/src/test/resources/");
        assertEquals(5, coverage.size());
        assertTrue(coverage.containsKey("fr.davidson.AppTest#testRandom"));
        assertTrue(coverage.get("fr.davidson.AppTest#testRandom").containsKey("/diff-jjoules-toy-java-project/src/main/java/fr/davidson/App.java"));
        final Integer[] expectedCoveredLines = new Integer[]{10, 11};
        assertEquals(expectedCoveredLines.length, coverage.get("fr.davidson.AppTest#testRandom").get("/diff-jjoules-toy-java-project/src/main/java/fr/davidson/App.java").size());
        for (Integer expectedCoveredLine : expectedCoveredLines) {
            assertTrue(coverage.get("fr.davidson.AppTest#testRandom").get("/diff-jjoules-toy-java-project/src/main/java/fr/davidson/App.java").contains(expectedCoveredLine));
        }
    }
}
