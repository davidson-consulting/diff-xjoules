package fr.davidson.diff_jjoules.utils;

import java.util.List;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 08/06/2022
 */
public class Constants {

    public static final String NEW_LINE = System.getProperty("line.separator");

    public static final String PATH_SEPARATOR = System.getProperty("path.separator");

    public static final String FILE_SEPARATOR = System.getProperty("file.separator");

    public static final String WHITE_SPACE = " ";

    public static String joinFiles(String... filenames) {
        return Constants.joinFiles(false, filenames);
    }

    public static String joinFiles(boolean shouldEndWithFileSeparator, String... filenames) {
        final String joinedFiles = String.join(FILE_SEPARATOR, filenames);
        return shouldEndWithFileSeparator && !joinedFiles.endsWith(FILE_SEPARATOR) ? joinedFiles + FILE_SEPARATOR : joinedFiles;
    }

    public static String joinFiles(List<String> filenames) {
        return joinFiles(filenames.toArray(new String[0]));
    }

    public static String joinPaths(String... pathnames) {
        return String.join(PATH_SEPARATOR, pathnames);
    }

    public static String joinPaths(List<String> pathnames) {
        return joinPaths(pathnames.toArray(new String[0]));
    }

    public static String joinLines(String... lines) {
        return String.join(NEW_LINE, lines);
    }

    public static String joinLines(List<String> lines) {
        return joinLines(lines.toArray(new String[0]));
    }

}
