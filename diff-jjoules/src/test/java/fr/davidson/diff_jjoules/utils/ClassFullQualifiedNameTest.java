package fr.davidson.diff_jjoules.utils;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 09/06/2022
 */
public class ClassFullQualifiedNameTest {

    @Test
    void testConstructorSingle() {
        final ClassFullQualifiedName classFullQualifiedName = new ClassFullQualifiedName("fr.davidson.diff_jjoules.utils.ClassFullQualifiedNameTest");
        assertEquals("fr.davidson.diff_jjoules.utils", classFullQualifiedName.packageName);
        assertEquals("ClassFullQualifiedNameTest", classFullQualifiedName.className);
        assertEquals("fr.davidson.diff_jjoules.utils.ClassFullQualifiedNameTest", classFullQualifiedName.toString());
    }

    @Test
    void testConstructor() {
        final ClassFullQualifiedName classFullQualifiedName = new ClassFullQualifiedName("fr.davidson.diff_jjoules.utils", "ClassFullQualifiedNameTest");
        assertEquals("fr.davidson.diff_jjoules.utils", classFullQualifiedName.packageName);
        assertEquals("ClassFullQualifiedNameTest", classFullQualifiedName.className);
        assertEquals("fr.davidson.diff_jjoules.utils.ClassFullQualifiedNameTest", classFullQualifiedName.toString());
    }
}
