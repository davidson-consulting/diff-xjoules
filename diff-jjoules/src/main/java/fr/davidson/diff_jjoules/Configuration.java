package fr.davidson.diff_jjoules;

import fr.davidson.diff_jjoules.steps.StepEnum;
import picocli.CommandLine;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
@CommandLine.Command(name = "fr.davidson.diff_jjoules.Main", mixinStandardHelpOptions = true, version = "Configuration 0.0.1")
public class Configuration {

    @CommandLine.Option(
            names = "--step",
            required = true,
            description = "Specify the step to perform." +
                    "Valid values: ${COMPLETION-CANDIDATES}"
    )
    private StepEnum step;

    @CommandLine.Option(names = {"-p", "--path-to-project"}, description = "Path to the program.", required = true)
    private String pathToProject;

    @CommandLine.Option(names = {"-o", "--output-path"}, description = "Path to the output.", required = true)
    private String outputPath;

    public StepEnum getStep() {
        return step;
    }

    public void setStep(StepEnum step) {
        this.step = step;
    }

    public String getPathToProject() {
        return pathToProject;
    }

    public void setPathToProject(String pathToProject) {
        this.pathToProject = pathToProject;
    }

    public String getOutputPath() {
        return outputPath;
    }

    public void setOutputPath(String outputPath) {
        this.outputPath = outputPath;
    }
}
