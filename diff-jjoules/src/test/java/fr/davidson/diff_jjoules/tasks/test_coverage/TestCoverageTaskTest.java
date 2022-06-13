package fr.davidson.diff_jjoules.tasks.test_coverage;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.Coverage;
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
public class TestCoverageTaskTest {

    @Test
    public void test() {
        final Configuration configuration = new Configuration();
        configuration.setPathToProject(new File("src/test/resources/diff-jjoules-toy-java-project").getAbsolutePath());
        configuration.setOutputPath("target/coverage.json");
        new TestCoverageTask().run(configuration);
        final Coverage coverage = JSONUtils.read("target/coverage.json", Coverage.class);
        assertTrue(coverage.containsTestIdentifier("fr.davidson.AppTest#testRandom"));
        assertTrue(coverage.findByTestIdentifier("fr.davidson.AppTest#testRandom").containsFileCoverage("src/main/java/fr/davidson/App.java"));
        final Integer[] expectedCoveredLines = new Integer[]{10, 11};
        assertEquals(expectedCoveredLines.length,
                coverage.findByTestIdentifier("fr.davidson.AppTest#testRandom")
                        .findFileCoverageByFilename("src/main/java/fr/davidson/App.java")
                        .covered_lines.size());
        for (Integer expectedCoveredLine : expectedCoveredLines) {
            assertTrue(
                    coverage.findByTestIdentifier("fr.davidson.AppTest#testRandom")
                            .findFileCoverageByFilename("src/main/java/fr/davidson/App.java")
                            .covered_lines
                            .contains(expectedCoveredLine)
            );
        }
    }
}
