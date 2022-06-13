package fr.davidson.diff_jjoules.tasks.test_coverage.coverage;

import java.util.ArrayList;
import java.util.List;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 13/06/2022
 */
public class FileCoverage {

    public final String filename;

    public final List<Integer> covered_lines;

    public FileCoverage(String filename) {
        this.filename = filename;
        this.covered_lines = new ArrayList<>();
    }

    public FileCoverage(String filename, List<Integer> covered_lines) {
        this.filename = filename;
        this.covered_lines = covered_lines;
    }
}
