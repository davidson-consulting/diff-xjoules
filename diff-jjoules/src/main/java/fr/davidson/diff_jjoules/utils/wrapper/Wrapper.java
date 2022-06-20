package fr.davidson.diff_jjoules.utils.wrapper;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 23/11/2021
 */
public interface Wrapper {

    public void clean(String pathToRootDir);

    public void compile(String pathToRootDir);

    public void cleanAndCompile(String pathToRootDir);

    public String buildClasspath(String pathToRootDir);

    public String getPathToSrcFolder();

    public String getPathToTestFolder();

    public String getPathToBinFolder();

    public String getPathToBinTestFolder();

    public String getBinaries();

    public void injectDependencies(String pathToRootDir);

    public void runClover(String pathToRootDif);

}