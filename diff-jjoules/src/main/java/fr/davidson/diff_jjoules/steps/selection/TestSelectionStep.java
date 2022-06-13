package fr.davidson.diff_jjoules.steps.selection;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.steps.Step;
import fr.davidson.diff_jjoules.steps.selection.coverage.CloverExecutor;
import fr.davidson.diff_jjoules.steps.selection.coverage.CloverReader;
import fr.davidson.diff_jjoules.steps.selection.coverage.Coverage;
import fr.davidson.diff_jjoules.utils.JSONUtils;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public class TestSelectionStep implements Step {

    @Override
    public void run(Configuration configuration) {
        final String pathToProject = configuration.getPathToProject();
        new CloverExecutor().instrumentAndRunTest(pathToProject);
        final Coverage coverage = new CloverReader().read(pathToProject);
        JSONUtils.write(configuration.getOutputPath(), coverage);
    }

}
