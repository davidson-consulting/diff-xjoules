package fr.davidson.diff_jjoules.tasks.test_execution;

import eu.stamp_project.testrunner.EntryPoint;
import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.tasks.Task;
import fr.davidson.diff_jjoules.utils.Constants;
import fr.davidson.diff_jjoules.utils.JSONUtils;
import fr.davidson.diff_jjoules.utils.TestsSet;
import fr.davidson.diff_jjoules.utils.wrapper.Wrapper;

import java.io.File;
import java.util.concurrent.TimeoutException;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 20/06/2022
 */
public class TestExecutionTask implements Task {
    @Override
    public void run(Configuration configuration) {
        final TestsSet testsSet = JSONUtils.read(configuration.getTestsSetPath(), TestsSet.class);
        final Wrapper wrapper = configuration.getWrapper();
        this._run(configuration.getPathToProjectV1(), wrapper, testsSet);
        this._run(configuration.getPathToProjectV2(), wrapper, testsSet);
    }

    private void _run(final String pathToProject, final Wrapper wrapper, final TestsSet testsSet) {
        EntryPoint.verbose = true;
        EntryPoint.timeoutInMs = 100000;
        EntryPoint.workingDirectory = new File(pathToProject);
        EntryPoint.nbFailingLoadClass = 5;
        try {
            wrapper.cleanAndCompile(pathToProject);
            final String classpath = wrapper.buildClasspath(pathToProject);
            EntryPoint.runTests(
                    Constants.joinPaths(wrapper.getBinaries(), classpath),
                    testsSet.getTestClassNames(),
                    testsSet.getTestMethodNames()
            );
        } catch (TimeoutException | java.lang.RuntimeException e) {
            throw new RuntimeException(e);
        }
    }
}
