package fr.davidson.diff_jjoules.utils;

import fr.davidson.diff_jjoules.Configuration;
import fr.davidson.diff_jjoules.steps.StepEnum;
import fr.davidson.diff_jjoules.steps.selection.TestSelectionStep;
import org.junit.jupiter.api.Test;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.assertEquals;


/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class JSONUtilsTest {

    @Test
    void testWrite() throws IOException {
        final String jsonFilePath = "target/test_write_json.json";
        final Configuration configuration = new Configuration();
        configuration.setPathToProject("this.is.a.path.to.project");
        configuration.setOutputPath("this.is.a.output.path");
        configuration.setStep(StepEnum.TEST_EXECUTION);
        JSONUtils.write(jsonFilePath, configuration);
        try (final BufferedReader reader = new BufferedReader(new FileReader(jsonFilePath))) {
            assertEquals(
                    EXPECTED_CONFIGURATION_JSON,
                    reader.lines().collect(Collectors.joining(Constants.NEW_LINE))
            );
        }
    }

    @Test
    void testRead() {
        final String jsonFilePath = "src/test/resources/test_read_json.json";
        final Configuration configurationLoadFromJSON = JSONUtils.read(jsonFilePath, Configuration.class);
        assertEquals("this.is.a.path.to.project", configurationLoadFromJSON.getPathToProject());
        assertEquals("this.is.a.output.path", configurationLoadFromJSON.getOutputPath());
        assertEquals(StepEnum.TEST_EXECUTION, configurationLoadFromJSON.getStep());
    }

    public static final String EXPECTED_CONFIGURATION_JSON = "{" + Constants.NEW_LINE +
            "  \"step\": \"TEST_EXECUTION\"," + Constants.NEW_LINE +
            "  \"pathToProject\": \"this.is.a.path.to.project\"," + Constants.NEW_LINE +
            "  \"outputPath\": \"this.is.a.output.path\"" + Constants.NEW_LINE +
            "}";
}
