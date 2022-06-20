package fr.davidson.diff_jjoules.tasks.test_execution;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.utils.wrapper.WrapperEnum;
import org.junit.jupiter.api.Test;

import java.io.File;

import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 20/06/2022
 */
public class TestExecutionTaskTest {

    @Test
    void test() {
        final Configuration configuration = new Configuration();
        configuration.setWrapperEnum(WrapperEnum.MAVEN);
        configuration.setPathToProject("src/test/resources/diff-jjoules-toy-java-project");
        configuration.setTestsSetPath("src/test/resources/test_selection.json");
        new TestExecutionTask().run(configuration);
    }
}
