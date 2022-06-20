package fr.davidson.diff_jjoules.utils.wrapper.properties;

import fr.davidson.diff_jjoules.utils.Constants;
import fr.davidson.diff_jjoules.utils.wrapper.Wrapper;
import fr.davidson.diff_jjoules.utils.wrapper.WrapperEnum;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.nio.file.Paths;

import static org.junit.jupiter.api.Assertions.*;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 24/11/2021
 */
public class PropertiesWrapperTest extends AbstractPropertiesWrapperTest {

    @BeforeEach
    void setUp() throws IOException {
        super.setUp();
        try {
            Files.walk(Paths.get("src/test/resources/diff-jjoules-toy-java-project/target"))
                    .map(Path::toFile)
                    .forEach(File::delete);
        } catch (NoSuchFileException ignored) {
            // ignored
        }
    }

    @Test
    void testCleanAndCompile() {
        final Wrapper wrapper = WrapperEnum.PROPERTIES.getWrapper();
        wrapper.cleanAndCompile("src/test/resources/diff-jjoules-toy-java-project/");
        assertTrue(new File("src/test/resources/diff-jjoules-toy-java-project/target/classes/").exists());
        assertTrue(new File("src/test/resources/diff-jjoules-toy-java-project/target/test-classes/").exists());
    }

    @Test
    void testClean() {
        final Wrapper wrapper = WrapperEnum.PROPERTIES.getWrapper();
        wrapper.clean("src/test/resources/diff-jjoules-toy-java-project/");
        assertFalse(new File("src/test/resources/diff-jjoules-toy-java-project/target/classes/").exists());
        assertFalse(new File("src/test/resources/diff-jjoules-toy-java-project/target/test-classes/").exists());
    }

    @Test
    void testCompile() {
        final Wrapper wrapper = WrapperEnum.PROPERTIES.getWrapper();
        wrapper.compile("src/test/resources/diff-jjoules-toy-java-project/");
        assertTrue(new File("src/test/resources/diff-jjoules-toy-java-project/target").exists());
    }

    @Test
    void testBuildClasspath() {
        final Wrapper wrapper = WrapperEnum.PROPERTIES.getWrapper();
        String classpath = wrapper.buildClasspath("src/test/resources/diff-jjoules-toy-java-project/");
        assertTrue(classpath.contains("junit/junit/4.11/junit-4.11.jar"));
        assertTrue(classpath.contains("org/hamcrest/hamcrest-core/1.3/hamcrest-core-1.3.jar"));
        classpath = wrapper.buildClasspath("src/test/resources/diff-jjoules-toy-java-project");
        assertTrue(classpath.contains("junit/junit/4.11/junit-4.11.jar"));
        assertTrue(classpath.contains("org/hamcrest/hamcrest-core/1.3/hamcrest-core-1.3.jar"));
    }

    @Test
    void testConstants() {
        final Wrapper wrapper = WrapperEnum.PROPERTIES.getWrapper();
        assertEquals(
                "src" + Constants.FILE_SEPARATOR + "main" + Constants.FILE_SEPARATOR + "java" + Constants.FILE_SEPARATOR + "",
                wrapper.getPathToSrcFolder()
        );
        assertEquals(
                "src" + Constants.FILE_SEPARATOR + "test" + Constants.FILE_SEPARATOR + "java" + Constants.FILE_SEPARATOR + "",
                wrapper.getPathToTestFolder()
        );
        assertEquals(
                "target" + Constants.FILE_SEPARATOR + "classes" + Constants.FILE_SEPARATOR + "",
                wrapper.getPathToBinFolder()
        );
        assertEquals(
                "target" + Constants.FILE_SEPARATOR + "test-classes" + Constants.FILE_SEPARATOR + "",
                wrapper.getPathToBinTestFolder()
        );
        assertEquals(
                "target" + Constants.FILE_SEPARATOR + "classes" + Constants.FILE_SEPARATOR + Constants.PATH_SEPARATOR
                        + "target" + Constants.FILE_SEPARATOR + "test-classes" + Constants.FILE_SEPARATOR + "",
                wrapper.getBinaries()
        );
    }

}
