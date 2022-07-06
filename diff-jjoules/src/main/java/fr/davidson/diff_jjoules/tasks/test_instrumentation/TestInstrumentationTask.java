package fr.davidson.diff_jjoules.tasks.test_instrumentation;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.tasks.Task;
import fr.davidson.diff_jjoules.utils.Constants;
import fr.davidson.diff_jjoules.utils.JSONUtils;
import fr.davidson.diff_jjoules.utils.TestsSet;
import fr.davidson.diff_jjoules.utils.wrapper.Wrapper;
import spoon.Launcher;
import spoon.OutputType;
import spoon.SpoonException;
import spoon.processing.AbstractProcessor;
import spoon.reflect.declaration.CtMethod;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 20/06/2022
 */
public class TestInstrumentationTask implements Task {


    @Override
    public void run(Configuration configuration) {
        final TestsSet testsSet = JSONUtils.read(configuration.getTestsSetPath(), TestsSet.class);
        final Wrapper wrapper = configuration.getWrapper();
        final String pathToProject = configuration.getPathToProject();
        wrapper.cleanAndCompile(pathToProject);
        final String classpath = wrapper.buildClasspath(pathToProject);

        final TestInstrumentationProcessor processor = new TestInstrumentationProcessor(
                testsSet,
                pathToProject,
                wrapper.getPathToTestFolder()
        );
        this.instrument(
                processor,
                classpath.split(Constants.PATH_SEPARATOR),
                pathToProject,
                wrapper
        );
        wrapper.injectDependencies(pathToProject);
    }

    private void instrument(AbstractProcessor<CtMethod<?>> processor, String[] classpath, String pathToProject, Wrapper wrapper) {
        Launcher launcher = new Launcher();

        final String[] finalClassPath = new String[classpath.length + 2];
        finalClassPath[0] = Constants.joinFiles(pathToProject, wrapper.getPathToBinFolder());
        finalClassPath[1] = Constants.joinFiles(pathToProject, wrapper.getPathToBinTestFolder());
        System.arraycopy(classpath, 0, finalClassPath, 2, classpath.length);
        launcher.getEnvironment().setSourceClasspath(finalClassPath);
        launcher.getEnvironment().setNoClasspath(true);
        launcher.getEnvironment().setAutoImports(false);
        launcher.getEnvironment().setLevel("DEBUG");
        launcher.addInputResource(pathToProject + Constants.FILE_SEPARATOR + wrapper.getPathToTestFolder());

        launcher.addProcessor(processor);
        launcher.getEnvironment().setOutputType(OutputType.NO_OUTPUT);
        try {
            launcher.buildModel();
            launcher.process();
        } catch (SpoonException sp) {
            throw new RuntimeException(sp);
        }
    }
}
