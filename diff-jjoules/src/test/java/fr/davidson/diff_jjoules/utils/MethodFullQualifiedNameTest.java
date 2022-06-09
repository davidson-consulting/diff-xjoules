package fr.davidson.diff_jjoules.utils;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class MethodFullQualifiedNameTest {

    @Test
    void testConstructorSingle() {
        final MethodFullQualifiedName methodFullQualifiedName = new MethodFullQualifiedName("fr.davidson.diff_jjoules.utils.MethodFullQualifiedNameTest#testConstructorSingle");
        assertEquals("fr.davidson.diff_jjoules.utils", methodFullQualifiedName.packageName);
        assertEquals("MethodFullQualifiedNameTest", methodFullQualifiedName.className);
        assertEquals("testConstructorSingle", methodFullQualifiedName.methodName);
        assertEquals("fr.davidson.diff_jjoules.utils.MethodFullQualifiedNameTest#testConstructorSingle", methodFullQualifiedName.toString());
    }

    @Test
    void testConstructorMethod() {
        final MethodFullQualifiedName methodFullQualifiedName = new MethodFullQualifiedName("fr.davidson.diff_jjoules.utils.MethodFullQualifiedNameTest", "testConstructorMethod");
        assertEquals("fr.davidson.diff_jjoules.utils", methodFullQualifiedName.packageName);
        assertEquals("MethodFullQualifiedNameTest", methodFullQualifiedName.className);
        assertEquals("testConstructorMethod", methodFullQualifiedName.methodName);
        assertEquals("fr.davidson.diff_jjoules.utils.MethodFullQualifiedNameTest#testConstructorMethod", methodFullQualifiedName.toString());
    }

    @Test
    void testConstructor() {
        final MethodFullQualifiedName methodFullQualifiedName = new MethodFullQualifiedName("fr.davidson.diff_jjoules.utils", "MethodFullQualifiedNameTest", "testConstructor");
        assertEquals("fr.davidson.diff_jjoules.utils", methodFullQualifiedName.packageName);
        assertEquals("MethodFullQualifiedNameTest", methodFullQualifiedName.className);
        assertEquals("testConstructor", methodFullQualifiedName.methodName);
        assertEquals("fr.davidson.diff_jjoules.utils.MethodFullQualifiedNameTest#testConstructor", methodFullQualifiedName.toString());
    }

}
