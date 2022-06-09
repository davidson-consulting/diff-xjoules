package fr.davidson.diff_jjoules.utils;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class ClassFullQualifiedName {

    public final String packageName;

    public final String className;

    public ClassFullQualifiedName(String fullQualifiedClassName) {
        final String[] splitFullQualifiedName = fullQualifiedClassName.split("\\.");
        final String[] splitPackageName = getPackageName(splitFullQualifiedName);
        this.packageName = String.join(".", splitPackageName);
        this.className = splitFullQualifiedName[splitFullQualifiedName.length - 1];
    }

    public ClassFullQualifiedName(String packageName, String className) {
        this.packageName = packageName;
        this.className = className;
    }

    private String[] getPackageName(String[] splitFullQualifiedName) {
        final String[] splitPackageName = new String[splitFullQualifiedName.length - 1];
        System.arraycopy(splitFullQualifiedName, 0, splitPackageName, 0, splitFullQualifiedName.length - 1);
        return splitPackageName;
    }

    public String toString() {
        return String.join(".", this.packageName, this.className);
    }

}
