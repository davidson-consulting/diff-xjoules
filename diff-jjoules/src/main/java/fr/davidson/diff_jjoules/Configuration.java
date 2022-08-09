package fr.davidson.diff_jjoules;

import fr.davidson.diff_jjoules.tasks.TaskEnum;
import fr.davidson.diff_jjoules.utils.wrapper.Wrapper;
import fr.davidson.diff_jjoules.utils.wrapper.WrapperEnum;
import picocli.CommandLine;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
@CommandLine.Command(name = "fr.davidson.diff_jjoules.Main", mixinStandardHelpOptions = true, version = "Configuration 0.0.1")
public class Configuration {

    @CommandLine.Option(
            names = "--task",
            required = true,
            description = "Specify the task to perform." +
                    "Valid values: ${COMPLETION-CANDIDATES}"
    )
    private TaskEnum task;

    @CommandLine.Option(names = {"-f", "--path-to-project-v1"}, description = "Path to the program in the first version.", required = true)
    private String pathToProjectV1;

    @CommandLine.Option(names = {"-s", "--path-to-project-v2"}, description = "Path to the program in the second version.", required = true)
    private String pathToProjectV2;

    @CommandLine.Option(names = {"-o", "--output-path"}, description = "Path to the output.", required = false)
    private String outputPath;

    @CommandLine.Option(names = {"-t", "--tests-set"}, description = "Path to the json file of tests set.", required = false)
    private String testsSetPath;

    @CommandLine.Option(
            names = "--wrapper",
            required = false,
            defaultValue = "MAVEN",
            description = "Specify the task to perform." +
                    "Valid values: ${COMPLETION-CANDIDATES}." +
                    "Default value: ${DEFAULT_VALUE}"
    )
    private WrapperEnum wrapperEnum;

    private Wrapper wrapper;

    public TaskEnum getTask() {
        return task;
    }

    public void setTask(TaskEnum task) {
        this.task = task;
    }

    public String getPathToProjectV1() {
        return pathToProjectV1;
    }

    public void setPathToProjectV1(String pathToProjectV1) {
        this.pathToProjectV1 = pathToProjectV1;
    }

    public String getPathToProjectV2() {
        return pathToProjectV2;
    }

    public void setPathToProjectV2(String pathToProjectV2) {
        this.pathToProjectV2 = pathToProjectV2;
    }

    public String getOutputPath() {
        return outputPath;
    }

    public void setOutputPath(String outputPath) {
        this.outputPath = outputPath;
    }

    public String getTestsSetPath() {
        return testsSetPath;
    }

    public void setTestsSetPath(String testsSetPath) {
        this.testsSetPath = testsSetPath;
    }

    public WrapperEnum getWrapperEnum() {
        return wrapperEnum;
    }

    /**
     * Reset also the field wrapper, see {@code getWrapper()}
     */
    public void setWrapperEnum(WrapperEnum wrapperEnum) {
        this.wrapperEnum = wrapperEnum;
        this.wrapper = null;
    }

    public Wrapper getWrapper() {
        if (this.wrapper == null) {
            this.wrapper = this.wrapperEnum.getWrapper();
        }
        return this.wrapper;
    }
}
