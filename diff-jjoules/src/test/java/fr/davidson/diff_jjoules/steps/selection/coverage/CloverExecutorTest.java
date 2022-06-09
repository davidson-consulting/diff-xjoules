package fr.davidson.diff_jjoules.steps.selection.coverage;

import org.junit.jupiter.api.Test;

import java.io.File;

import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class CloverExecutorTest {

    @Test
    void test() {
        final String pathFolderRoot = "src/test/resources/diff-jjoules-toy-java-project";
        new CloverExecutor()
                .instrumentAndRunTest(new File(pathFolderRoot).getAbsolutePath());
        assertTrue(new File(pathFolderRoot + "/target/clover/").exists());
        assertTrue(new File(pathFolderRoot + "/target/clover/").isDirectory());
        assertTrue(new File(pathFolderRoot + "/target/clover/clover.db").exists());
    }
}
