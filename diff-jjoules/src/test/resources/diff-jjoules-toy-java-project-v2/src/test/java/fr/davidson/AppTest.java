package fr.davidson;
public class AppTest {
    @org.junit.Test
    public void testAddedStatement() {
        fr.davidson.tlpc.sensor.TLPCSensor.start("fr.davidson.AppTest#testAddedStatement");
        new fr.davidson.App().addedStatement();
        fr.davidson.tlpc.sensor.TLPCSensor.stop("fr.davidson.AppTest#testAddedStatement");
    }

    @org.junit.Test
    public void testRemovedStatement() {
        fr.davidson.tlpc.sensor.TLPCSensor.start("fr.davidson.AppTest#testRemovedStatement");
        new fr.davidson.App().removedStatement();
        fr.davidson.tlpc.sensor.TLPCSensor.stop("fr.davidson.AppTest#testRemovedStatement");
    }

    @org.junit.Test
    public void testUpdatedStatement() {
        fr.davidson.tlpc.sensor.TLPCSensor.start("fr.davidson.AppTest#testUpdatedStatement");
        new fr.davidson.App().updatedStatement();
        fr.davidson.tlpc.sensor.TLPCSensor.stop("fr.davidson.AppTest#testUpdatedStatement");
    }

    @org.junit.Test
    public void testAddedAndRemovedStatement() {
        fr.davidson.tlpc.sensor.TLPCSensor.start("fr.davidson.AppTest#testAddedAndRemovedStatement");
        new fr.davidson.App().addedAndRemovedStatement();
        fr.davidson.tlpc.sensor.TLPCSensor.stop("fr.davidson.AppTest#testAddedAndRemovedStatement");
    }

    @org.junit.Test
    public void testEmpty() {
        final java.util.List<java.lang.Integer> emptyList = java.util.Collections.emptyList();
        fr.davidson.App.sort(emptyList);
        org.junit.Assert.assertTrue(emptyList.isEmpty());
    }

    @org.junit.Test
    public void testSingleton() {
        final java.util.List<java.lang.Integer> singletonList = java.util.Collections.singletonList(23);
        fr.davidson.App.sort(singletonList);
        org.junit.Assert.assertTrue(fr.davidson.AppTest.isSorted(singletonList));
    }

    @org.junit.Test
    public void testRandom() {
        final java.util.List<java.lang.Integer> randomList = getRandomList(100);
        fr.davidson.App.sort(randomList);
        org.junit.Assert.assertTrue(fr.davidson.AppTest.isSorted(randomList));
    }

    @org.junit.Test
    public void testRandomQuickSort() {
        final java.util.List<java.lang.Integer> randomList = getRandomList(100);
        final java.util.List<java.lang.Integer> sortedRandomList = fr.davidson.App.quickSort(randomList);
        org.junit.Assert.assertTrue(fr.davidson.AppTest.isSorted(sortedRandomList));
    }

    @org.junit.Test
    public void testRandomQuickSortLarge() {
        final java.util.List<java.lang.Integer> randomList = getRandomList(100000);
        final java.util.List<java.lang.Integer> sortedRandomList = fr.davidson.App.quickSort(randomList);
        org.junit.Assert.assertTrue(fr.davidson.AppTest.isSorted(sortedRandomList));
    }

    private java.util.List<java.lang.Integer> getRandomList(final int bound) {
        final java.util.List<java.lang.Integer> randomList = new java.util.ArrayList<>();
        final java.util.Random random = new java.util.Random();
        final int nbRandomElement = random.nextInt(bound);
        for (int i = 0; i < nbRandomElement; i++) {
            randomList.add(random.nextInt());
        }
        return randomList;
    }

    private static boolean isSorted(java.util.List<java.lang.Integer> list) {
        int previous = list.get(0);
        for (int i = 1; i < list.size(); i++) {
            int current = list.get(i);
            if (current < previous) {
                return false;
            }
            previous = current;
        }
        return true;
    }

    static {
           java.lang.Runtime.getRuntime().addShutdownHook(       new java.lang.Thread() {           @java.lang.Override           public void run() {               fr.davidson.tlpc.sensor.TLPCSensor.report(                   "/home/benjamin/workspace/diff-xjoules/diff-jjoules/src/test/resources/diff-jjoules-toy-java-project-v2/diff-measurements/measurements.json"               );           }       }   );
    }
}