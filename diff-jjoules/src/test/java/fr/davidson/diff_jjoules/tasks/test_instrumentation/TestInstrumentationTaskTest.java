package fr.davidson.diff_jjoules.tasks.test_instrumentation;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.utils.wrapper.WrapperEnum;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 20/06/2022
 */
public class TestInstrumentationTaskTest {

    @BeforeEach
    void setUp() throws IOException {
        Files.copy(
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/src/test/java/fr/davidson/AppTest.java"),
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/AppTest.java.backup"),
                StandardCopyOption.REPLACE_EXISTING
        );
        Files.copy(
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/pom.xml"),
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/pom.xml.backup"),
                StandardCopyOption.REPLACE_EXISTING
        );
    }

    @Test
    void test() {
        final Configuration configuration = new Configuration();
        configuration.setPathToProject("src/test/resources/diff-jjoules-toy-java-project");
        configuration.setWrapperEnum(WrapperEnum.MAVEN);
        configuration.setTestsSetPath("src/test/resources/test_selection.json");
        new TestInstrumentationTask().run(configuration);
    }

    @AfterEach
    void tearDown() throws IOException {
        Files.copy(
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/AppTest.java.backup"),
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/src/test/java/fr/davidson/AppTest.java"),
                StandardCopyOption.REPLACE_EXISTING
        );
        Files.copy(
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/pom.xml.backup"),
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/pom.xml"),
                StandardCopyOption.REPLACE_EXISTING
        );
    }
}
