package fr.davidson.diff_jjoules.tasks.test_coverage;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.tasks.Task;
import fr.davidson.diff_jjoules.tasks.test_coverage.clover.CloverExecutor;
import fr.davidson.diff_jjoules.tasks.test_coverage.clover.CloverReader;
import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.Coverage;
import fr.davidson.diff_jjoules.utils.JSONUtils;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public class TestCoverageTask implements Task {

    @Override
    public void run(Configuration configuration) {
        final String pathToProject = configuration.getPathToProject();
        new CloverExecutor().instrumentAndRunTest(configuration.getWrapper(), pathToProject);
        final Coverage coverage = new CloverReader().read(pathToProject);
        JSONUtils.write(configuration.getOutputPath(), coverage);
    }

}
