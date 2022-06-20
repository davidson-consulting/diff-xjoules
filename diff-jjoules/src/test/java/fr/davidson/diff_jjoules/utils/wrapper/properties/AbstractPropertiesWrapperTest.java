package fr.davidson.diff_jjoules.utils.wrapper.properties;

import fr.davidson.diff_jjoules.utils.Constants;
import org.junit.jupiter.api.BeforeEach;

import java.io.FileWriter;
import java.io.IOException;
import java.util.Arrays;
import java.util.stream.Collectors;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 26/11/2021
 */
public class AbstractPropertiesWrapperTest {

    private final String[] CLASSPATH_ELEMENTS = new String[]{
            "junit/junit/4.11/junit-4.11.jar",
            "org/hamcrest/hamcrest-core/1.3/hamcrest-core-1.3.jar"
    };

    @BeforeEach
    void setUp() throws IOException {
        try (final FileWriter write = new FileWriter("src/test/resources/diff-jjoules-toy-java-project/classpath", false)) {
            final String mavenHome = System.getProperty("user.home") + "/.m2/repository/";
            write.write(
                    Arrays.stream(CLASSPATH_ELEMENTS)
                            .map(classpathElement -> mavenHome + Constants.FILE_SEPARATOR + classpathElement)
                            .collect(Collectors.joining(Constants.PATH_SEPARATOR))
            );
        }
    }
}
