package fr.davidson.diff_jjoules.utils;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class MethodFullQualifiedName extends ClassFullQualifiedName {

    public final String methodName;

    public MethodFullQualifiedName(String fullQualifiedClassName, String methodName) {
        super(fullQualifiedClassName);
        this.methodName = methodName;
    }

    public MethodFullQualifiedName(String packageName, String className, String methodName) {
        super(packageName, className);
        this.methodName = methodName;
    }

    public MethodFullQualifiedName(String methodFullQualifiedName) {
        this(methodFullQualifiedName.split("#")[0], methodFullQualifiedName.split("#")[1]);
    }

    public String getClassFullQualifiedName() {
        return super.toString();
    }

    public String toString() {
        return super.toString() + "#" + this.methodName;
    }

}
