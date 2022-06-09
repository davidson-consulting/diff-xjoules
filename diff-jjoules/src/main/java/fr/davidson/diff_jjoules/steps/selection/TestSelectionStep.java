package fr.davidson.diff_jjoules.steps.selection;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.steps.Step;
import fr.davidson.diff_jjoules.steps.selection.coverage.CloverExecutor;
import fr.davidson.diff_jjoules.steps.selection.coverage.CloverReader;
import fr.davidson.diff_jjoules.utils.JSONUtils;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

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

    // Associate a test identifier to the association between a filename and the covered lines
    public static class Coverage extends HashMap<String, Map<String, List<Integer>>>{};

}
