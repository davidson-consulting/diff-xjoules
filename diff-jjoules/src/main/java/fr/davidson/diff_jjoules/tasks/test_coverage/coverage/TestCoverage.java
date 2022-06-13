package fr.davidson.diff_jjoules.tasks.test_coverage.coverage;

import java.util.ArrayList;
import java.util.List;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 13/06/2022
 */
public class TestCoverage {

    public final String test_identifier;

    public final List<FileCoverage> file_coverages;

    public TestCoverage(String test_identifier) {
        this.test_identifier = test_identifier;
        this.file_coverages = new ArrayList<>();
    }

    public TestCoverage(String test_identifier, List<FileCoverage> file_coverages) {
        this.test_identifier = test_identifier;
        this.file_coverages = file_coverages;
    }

    public boolean containsFileCoverage(String filename) {
        return this.findFileCoverageByFilename(filename) != null;
    }

    public FileCoverage findFileCoverageByFilename(String filename) {
        return this.file_coverages.stream()
                .filter(fileCoverage -> filename.equals(fileCoverage.filename))
                .findFirst()
                .orElse(null);
    }
}
