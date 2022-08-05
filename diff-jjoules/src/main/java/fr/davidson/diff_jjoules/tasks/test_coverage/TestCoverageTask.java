package fr.davidson.diff_jjoules.tasks.test_coverage;

import eu.stamp_project.testrunner.EntryPoint;
import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.tasks.Task;
import fr.davidson.diff_jjoules.tasks.test_coverage.clover.CloverExecutor;
import fr.davidson.diff_jjoules.tasks.test_coverage.clover.CloverReader;
import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.Coverage;
import fr.davidson.diff_jjoules.utils.Constants;
import fr.davidson.diff_jjoules.utils.JSONUtils;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public class TestCoverageTask implements Task {

    @Override
    public void run(Configuration configuration) {
        this.run(configuration, configuration.getPathToProject(), "coverage_v1.json");
        this.run(configuration, configuration.getPathToProjectSecondVersion(), "coverage_v2.json");
    }

    private void run(final Configuration configuration, final String pathToProject, final String jsonFilename) {
        new CloverExecutor().instrumentAndRunTest(configuration.getWrapper(), pathToProject);
        final Coverage coverage = new CloverReader().read(pathToProject);
        JSONUtils.write(Constants.joinFiles(configuration.getOutputPath(), jsonFilename), coverage);
    }

}
