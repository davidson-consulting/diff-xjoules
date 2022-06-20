package fr.davidson.diff_jjoules.utils.wrapper.properties;

import fr.davidson.diff_jjoules.utils.wrapper.Wrapper;
import fr.davidson.diff_jjoules.utils.wrapper.WrapperEnum;
import org.junit.Ignore;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 26/11/2021
 */
public class PropertiesWrapperInjectionTest extends AbstractPropertiesWrapperTest {

    @BeforeEach
    void setUp() throws IOException {
        Files.copy(
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/classpath"),
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/classpath.backup"),
                StandardCopyOption.REPLACE_EXISTING
        );
    }

    @AfterEach
    void tearDown() throws IOException {
        Files.copy(
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/classpath.backup"),
                Paths.get("src/test/resources/diff-jjoules-toy-java-project/classpath"),
                StandardCopyOption.REPLACE_EXISTING
        );
    }

    @Ignore
    @Test
    void testInjection() {
        // TODO this is one requires to have TLPCSensor as dependency.
        // TODO For now we want to avoid such situation.
        final Wrapper wrapper = WrapperEnum.PROPERTIES.getWrapper();
        try (final BufferedReader reader = new BufferedReader(new FileReader(("src/test/resources/diff-jjoules-toy-java-project/classpath")))) {
            assertFalse(reader.lines().anyMatch(line -> line.contains("tlpc.sensor")));
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
        wrapper.injectDependencies("src/test/resources/diff-jjoules-toy-java-project/");
        try (final BufferedReader reader = new BufferedReader(new FileReader(("src/test/resources/diff-jjoules-toy-java-project/classpath")))) {
            assertTrue(reader.lines().anyMatch(line -> line.contains("tlpc.sensor")));
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

}
