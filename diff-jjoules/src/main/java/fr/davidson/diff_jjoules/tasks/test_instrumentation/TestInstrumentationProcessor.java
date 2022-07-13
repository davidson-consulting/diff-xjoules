package fr.davidson.diff_jjoules.tasks.test_instrumentation;

import eu.stamp_project.testrunner.test_framework.TestFramework;
import fr.davidson.diff_jjoules.utils.ClassFullQualifiedName;
import fr.davidson.diff_jjoules.utils.Constants;
import fr.davidson.diff_jjoules.utils.MethodFullQualifiedName;
import fr.davidson.diff_jjoules.utils.TestsSet;
import org.apache.commons.io.FileUtils;
import spoon.processing.AbstractProcessor;
import spoon.reflect.declaration.*;
import spoon.reflect.factory.Factory;
import spoon.reflect.visitor.PrettyPrinter;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.*;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 20/06/2022
 */
public class TestInstrumentationProcessor extends AbstractProcessor<CtMethod<?>> {

    public static final String FOLDER_MEASURES_PATH = "diff-measurements";

    public static final String OUTPUT_FILE_NAME = "measurements.json";

    private final Set<CtType<?>> instrumentedTypes;

    private final TestsSet testsToBeInstrumented;

    private final Map<String, List<String>> testMethodsPerTestClassToBeInstrumented;

    private final String rootPathFolder;

    private final String testFolderPath;

    public TestInstrumentationProcessor(
            TestsSet testsList,
            String rootPathFolder,
            String testFolderPath) {
        this.instrumentedTypes = new HashSet<>();
        this.testsToBeInstrumented = testsList;
        this.testMethodsPerTestClassToBeInstrumented = new HashMap<>();
        for (String testIdentifier : testsList.test_selection) {
            final MethodFullQualifiedName methodFullQualifiedName = new MethodFullQualifiedName(testIdentifier);
            final String classFullQualifiedName = methodFullQualifiedName.getClassFullQualifiedName();
            if (!this.testMethodsPerTestClassToBeInstrumented.containsKey(classFullQualifiedName)) {
                this.testMethodsPerTestClassToBeInstrumented.put(classFullQualifiedName, new ArrayList<>());
            }
            this.testMethodsPerTestClassToBeInstrumented.get(classFullQualifiedName).add(methodFullQualifiedName.methodName);
        }
        this.rootPathFolder = new File(rootPathFolder).getAbsolutePath();
        this.testFolderPath = testFolderPath;
    }

    @Override
    public boolean isToBeProcessed(CtMethod<?> candidate) {
        if ((!this.testMethodsPerTestClassToBeInstrumented.isEmpty()) && this.testMethodsPerTestClassToBeInstrumented.values()
                .stream()
                .noneMatch(tests -> tests.contains(candidate.getSimpleName()))) {
            return false;
        }
        CtType<?> declaringType = candidate.getDeclaringType();
        if (declaringType == null) {
            return false;
        }
        if (candidate.getModifiers().contains(ModifierKind.PRIVATE) || !candidate.getParameters().isEmpty()) {
            return false;
        }
        TestFramework.init(candidate.getFactory());
        return (TestFramework.isJUnit4(candidate) || TestFramework.isJUnit5(candidate)) &&
                (
                        this.mustInstrument(declaringType.getQualifiedName(), candidate.getSimpleName()) ||
                                this.checkInheritance(candidate)
                );
    }

    private boolean mustInstrument(String testClassQualifiedName, String testMethodName) {
        return this.testMethodsPerTestClassToBeInstrumented.isEmpty() || (
                this.testMethodsPerTestClassToBeInstrumented.containsKey(testClassQualifiedName) &&
                        this.testMethodsPerTestClassToBeInstrumented
                                .get(testClassQualifiedName)
                                .contains(testMethodName));
    }

    private boolean checkInheritance(CtMethod<?> candidate) {
        final CtType<?> declaringType = candidate.getDeclaringType();
        return candidate.getFactory().Type().getAll()
                .stream()
                .filter(type -> type.getSuperclass() != null)
                .filter(type -> type.getSuperclass().getDeclaration() != null)
                .filter(type -> type.getSuperclass().getTypeDeclaration().equals(declaringType))
                .anyMatch(ctType -> this.mustInstrument(ctType.getQualifiedName(), candidate.getSimpleName()));
    }

    @Override
    public void processingDone() {
        this.addShutdownHookToReport(this.instrumentedTypes.stream().findFirst().get());
        this.instrumentedTypes.forEach(this::processingDone);
        this.instrumentedTypes.clear();
    }

    protected void processingDone(CtType<?> type) {
        this.printCtType(type);
        final File outputMeasureFd = new File(this.rootPathFolder + Constants.FILE_SEPARATOR + FOLDER_MEASURES_PATH + Constants.FILE_SEPARATOR);
        if (outputMeasureFd.exists()) {
            try {
                FileUtils.deleteDirectory(outputMeasureFd);
            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        }
        outputMeasureFd.mkdir();
    }

    private void addShutdownHookToReport(CtType<?> type) {
        final Factory factory = type.getFactory();
        final CtAnonymousExecutable anonymousExecutable = factory.createAnonymousExecutable();
        final String outputPathname = Constants.joinFiles(this.rootPathFolder, FOLDER_MEASURES_PATH, OUTPUT_FILE_NAME);
        anonymousExecutable.setBody(factory.createCodeSnippetStatement(
                "   java.lang.Runtime.getRuntime().addShutdownHook(" +
                        "       new java.lang.Thread() {" +
                        "           @java.lang.Override" +
                        "           public void run() {" +
                        "               fr.davidson.tlpc.sensor.TLPCSensor.report(" +
                        "                   \"" + outputPathname + "\"" +
                        "               );" +
                        "           }" +
                        "       }" +
                        "   )"
        ));
        anonymousExecutable.setModifiers(Collections.singleton(ModifierKind.STATIC));
        type.addTypeMember(anonymousExecutable);
    }

    protected void printCtType(CtType<?> type) {
        final File directory = new File(this.rootPathFolder + Constants.FILE_SEPARATOR + this.testFolderPath);
        type.getFactory().getEnvironment().setSourceOutputDirectory(directory);
        final PrettyPrinter prettyPrinter = type.getFactory().getEnvironment().createPrettyPrinter();
        final String fileName = this.rootPathFolder +  Constants.FILE_SEPARATOR  +
                testFolderPath +  Constants.FILE_SEPARATOR  +
                type.getQualifiedName().replaceAll("\\.",  Constants.FILE_SEPARATOR ) + ".java";
        try (final FileWriter write = new FileWriter(fileName)) {
            write.write(prettyPrinter.printTypes(type));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void process(CtMethod<?> ctMethod) {
        final Factory factory = ctMethod.getFactory();
        final CtClass<?> parentClass = ctMethod.getParent(CtClass.class);
        final String identifier = new MethodFullQualifiedName(parentClass.getQualifiedName(), ctMethod.getSimpleName()).toString();
        ctMethod.getBody().insertBegin(
                factory.createCodeSnippetStatement("fr.davidson.tlpc.sensor.TLPCSensor.start(\"" + identifier + "\")")
        );
        ctMethod.getBody().insertEnd(
                factory.createCodeSnippetStatement("fr.davidson.tlpc.sensor.TLPCSensor.stop(\"" + identifier + "\")")
        );
        this.instrumentedTypes.add(ctMethod.getDeclaringType());
    }
}
