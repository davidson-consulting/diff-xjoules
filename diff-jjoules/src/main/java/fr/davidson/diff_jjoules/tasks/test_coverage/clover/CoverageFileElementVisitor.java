package fr.davidson.diff_jjoules.tasks.test_coverage.clover;

import com.atlassian.clover.api.registry.*;
import com.atlassian.clover.registry.FileElementVisitor;
import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.Coverage;
import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.FileCoverage;
import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.TestCoverage;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class CoverageFileElementVisitor implements FileElementVisitor {

    private final Coverage coverage;

    private final String testIdentifier;

    private final String targetFileName;

    public CoverageFileElementVisitor(
            Coverage coverage,
            String targetFileName,
            String testIdentifier) {
        this.targetFileName = targetFileName;
        this.testIdentifier = testIdentifier;
        this.coverage = coverage;
        if (!this.coverage.containsTestIdentifier(this.testIdentifier)) {
            this.coverage.test_coverages.add(new TestCoverage(this.testIdentifier));
        }
        this.coverage.findByTestIdentifier(this.testIdentifier).file_coverages.add(new FileCoverage(this.targetFileName));
    }

    @Override
    public void visitClass(ClassInfo info) {

    }

    @Override
    public void visitMethod(MethodInfo info) {
        this.addCoveredLine(info);
    }

    @Override
    public void visitStatement(StatementInfo info) {
        visitNode(info);
    }

    @Override
    public void visitBranch(BranchInfo info) {
        visitNode(info);
    }

    private void visitNode(ElementInfo info) {
        this.addCoveredLine(info);
    }

    private void addCoveredLine(ElementInfo info) {
        if (info.getHitCount() > 0) {
            this.coverage.findByTestIdentifier(this.testIdentifier)
                    .findFileCoverageByFilename(this.targetFileName)
                    .covered_lines.add(info.getStartLine());
        }
    }
}
