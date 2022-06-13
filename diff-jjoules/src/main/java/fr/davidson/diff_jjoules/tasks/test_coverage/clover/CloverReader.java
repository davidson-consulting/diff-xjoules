package fr.davidson.diff_jjoules.tasks.test_coverage.clover;

import clover.com.google.common.collect.Sets;
import com.atlassian.clover.BitSetCoverageProvider;
import com.atlassian.clover.CloverDatabase;
import com.atlassian.clover.CoverageData;
import com.atlassian.clover.CoverageDataSpec;
import com.atlassian.clover.api.CloverException;
import com.atlassian.clover.api.registry.FileInfo;
import com.atlassian.clover.api.registry.PackageInfo;
import com.atlassian.clover.registry.entities.FullFileInfo;
import com.atlassian.clover.registry.entities.FullPackageInfo;
import com.atlassian.clover.registry.entities.TestCaseInfo;
import com.atlassian.clover.registry.metrics.HasMetricsFilter;
import fr.davidson.diff_jjoules.tasks.test_coverage.coverage.Coverage;
import fr.davidson.diff_jjoules.utils.MethodFullQualifiedName;

import java.io.File;
import java.util.*;


/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public class CloverReader {

    private static final String ROOT_DIRECTORY = "/target/clover";

    private static final String DATABASE_FILE = "/clover.db";

    public Coverage read(String pathToProject) {
        final Coverage coverage = new Coverage();
        try {
            final CloverDatabase cloverDatabase = this.getCloverDatabase(pathToProject);
            for (PackageInfo packageInfo : cloverDatabase.getFullModel().getAllPackages()) {
                processPackage(pathToProject, cloverDatabase, (FullPackageInfo)packageInfo, coverage);
            }
        } catch (CloverException e) {
            throw new RuntimeException(e);
        }
        return coverage;
    }

    private void processPackage(String pathToProject, CloverDatabase database, FullPackageInfo packageInfo, Coverage coverage) {
        for (FileInfo file : packageInfo.getFiles()) {
            processFile(pathToProject, database, (FullFileInfo)file, coverage);
        }
    }

    private void processFile(String pathToProject, CloverDatabase database, FullFileInfo fileInfo, Coverage coverage) {
        final Set<TestCaseInfo> testHits = database.getTestHits(fileInfo);
        final FullFileInfo fcopy = fileInfo.copy((FullPackageInfo) fileInfo.getContainingPackage(), HasMetricsFilter.ACCEPT_ALL);
        final Set<TestCaseInfo> testSet = Sets.newHashSet();
        for (TestCaseInfo tci : testHits) {
            testSet.clear();
            testSet.add(tci);
            final CoverageData data = database.getCoverageData();
            fcopy.setDataProvider(new BitSetCoverageProvider(data.getHitsFor(testSet, fcopy), data));
            final String testIdentifier = new MethodFullQualifiedName(tci.getRuntimeTypeName(), tci.getTestName()).toString();
            final String targetFileName = fileInfo.getPhysicalFile().getAbsolutePath().substring(pathToProject.length() + 1);
            fcopy.visitElements(new CoverageFileElementVisitor(coverage, targetFileName, testIdentifier));
        }
    }

    private CloverDatabase getCloverDatabase(String directory) throws CloverException {
        final File rootDirectoryOfCloverFiles = new File(directory + ROOT_DIRECTORY);
        CloverDatabase database = new CloverDatabase(rootDirectoryOfCloverFiles.getAbsolutePath() + DATABASE_FILE);
        CoverageDataSpec spec = new CoverageDataSpec();
        database.loadCoverageData(spec);
        return database;
    }

}