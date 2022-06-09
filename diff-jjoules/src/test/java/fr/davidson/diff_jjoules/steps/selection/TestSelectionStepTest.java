package fr.davidson.diff_jjoules.steps.selection;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.utils.JSONUtils;
import org.junit.jupiter.api.Test;

import java.io.File;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class TestSelectionStepTest {

    @Test
    public void test() {
        final Configuration configuration = new Configuration();
        configuration.setPathToProject(new File("src/test/resources/diff-jjoules-toy-java-project").getAbsolutePath());
        configuration.setOutputPath("target/coverage.json");
        new TestSelectionStep().run(configuration);
        final TestSelectionStep.Coverage coverage = JSONUtils.read("target/coverage.json", TestSelectionStep.Coverage.class);
        assertEquals(5, coverage.size());
        assertTrue(coverage.containsKey("fr.davidson.AppTest#testRandom"));
        assertTrue(coverage.get("fr.davidson.AppTest#testRandom").containsKey("src/main/java/fr/davidson/App.java"));
        final Integer[] expectedCoveredLines = new Integer[]{10, 11};
        assertEquals(expectedCoveredLines.length, coverage.get("fr.davidson.AppTest#testRandom").get("src/main/java/fr/davidson/App.java").size());
        for (Integer expectedCoveredLine : expectedCoveredLines) {
            assertTrue(coverage.get("fr.davidson.AppTest#testRandom").get("src/main/java/fr/davidson/App.java").contains(expectedCoveredLine));
        }
    }
}
