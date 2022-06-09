package fr.davidson.diff_jjoules.steps.selection.coverage;

import com.atlassian.clover.api.registry.*;
import com.atlassian.clover.registry.FileElementVisitor;
import fr.davidson.diff_jjoules.steps.selection.TestSelectionStep;

import java.util.*;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class CoverageFileElementVisitor implements FileElementVisitor {

    private final TestSelectionStep.Coverage coverage;

    private final String testIdentifier;

    private final String targetFileName;

    public CoverageFileElementVisitor(
            TestSelectionStep.Coverage coverage,
            String targetFileName,
            String testIdentifier) {
        this.targetFileName = targetFileName;
        this.testIdentifier = testIdentifier;
        this.coverage = coverage;
        if (!this.coverage.containsKey(this.testIdentifier)) {
            this.coverage.put(this.testIdentifier, new HashMap<>());
        }
        this.coverage.get(this.testIdentifier).put(this.targetFileName, new ArrayList<>());
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
            this.coverage.get(this.testIdentifier).get(this.targetFileName).add(info.getStartLine());
        }
    }
}
