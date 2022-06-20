package fr.davidson.diff_jjoules.tasks.test_coverage.clover;

import fr.davidson.diff_jjoules.utils.wrapper.Wrapper;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public class CloverExecutor {

    public void instrumentAndRunTest(Wrapper wrapper, String pathToRootOfProject) {
        wrapper.runClover(pathToRootOfProject);
    }
}