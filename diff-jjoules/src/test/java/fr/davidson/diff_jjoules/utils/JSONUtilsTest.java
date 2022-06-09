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
        final TestSelectionStep.Coverage coverage = new TestSelectionStep.Coverage();
        coverage.put("key1", new HashMap<>());
        coverage.get("key1").put("key11", new ArrayList<>());
        coverage.get("key1").get("key11").add(1);
        coverage.get("key1").get("key11").add(2);
        coverage.get("key1").get("key11").add(3);
        coverage.get("key1").put("key12", new ArrayList<>());
        coverage.get("key1").get("key12").add(1);
        coverage.get("key1").get("key12").add(2);
        coverage.get("key1").get("key12").add(3);
        coverage.put("key2", new HashMap<>());
        coverage.get("key2").put("key21", new ArrayList<>());
        coverage.get("key2").get("key21").add(1);
        coverage.get("key2").get("key21").add(2);
        coverage.get("key2").get("key21").add(3);
        JSONUtils.write(jsonFilePath, coverage);
        try (final BufferedReader reader = new BufferedReader(new FileReader(jsonFilePath))) {
            assertEquals(
                    EXPECTED_COVERAGE_JSON,
                    reader.lines().collect(Collectors.joining(Constants.NEW_LINE))
            );
        }
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

    public static final String EXPECTED_COVERAGE_JSON = "{" + Constants.NEW_LINE +
            "  \"key1\": {" + Constants.NEW_LINE +
            "    \"key12\": [" + Constants.NEW_LINE +
            "      1," + Constants.NEW_LINE +
            "      2," + Constants.NEW_LINE +
            "      3" + Constants.NEW_LINE +
            "    ]," + Constants.NEW_LINE +
            "    \"key11\": [" + Constants.NEW_LINE +
            "      1," + Constants.NEW_LINE +
            "      2," + Constants.NEW_LINE +
            "      3" + Constants.NEW_LINE +
            "    ]" + Constants.NEW_LINE +
            "  }," + Constants.NEW_LINE +
            "  \"key2\": {" + Constants.NEW_LINE +
            "    \"key21\": [" + Constants.NEW_LINE +
            "      1," + Constants.NEW_LINE +
            "      2," + Constants.NEW_LINE +
            "      3" + Constants.NEW_LINE +
            "    ]" + Constants.NEW_LINE +
            "  }" + Constants.NEW_LINE +
            "}";

    public static final String EXPECTED_CONFIGURATION_JSON = "{" + Constants.NEW_LINE +
            "  \"step\": \"TEST_EXECUTION\"," + Constants.NEW_LINE +
            "  \"pathToProject\": \"this.is.a.path.to.project\"," + Constants.NEW_LINE +
            "  \"outputPath\": \"this.is.a.output.path\"" + Constants.NEW_LINE +
            "}";
}
