package fr.davidson;

import org.junit.Test;

import static org.junit.Assert.assertTrue;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Random;

public class AppTest {

    @Test
    public void testAddedStatement() {
        new App().addedStatement();
    }

    @Test
    public void testRemovedStatement() {
        new App().removedStatement();
    }

    @Test
    public void testUpdatedStatement() {
        new App().updatedStatement();
    }

    @Test
    public void testAddedAndRemovedStatement() {
        new App().addedAndRemovedStatement();
    }

    @Test
    public void testEmpty() {
        final List<Integer> emptyList = Collections.emptyList();
        App.sort(emptyList);
        assertTrue(emptyList.isEmpty());
    }

    @Test
    public void testSingleton() {
        final List<Integer> singletonList = Collections.singletonList(23);
        App.sort(singletonList);
        assertTrue(isSorted(singletonList));
    }

    @Test
    public void testRandom() {
        final List<Integer> randomList = getRandomList(100);
        App.sort(randomList);
        assertTrue(isSorted(randomList));
    }

    @Test
    public void testRandomQuickSort() {
        final List<Integer> randomList = getRandomList(100);
        final List<Integer> sortedRandomList = App.quickSort(randomList);
        assertTrue(isSorted(sortedRandomList));
    }

    @Test
    public void testRandomQuickSortLarge() {
        final List<Integer> randomList = getRandomList(100000);
        final List<Integer> sortedRandomList = App.quickSort(randomList);
        assertTrue(isSorted(sortedRandomList));
    }

    private List<Integer> getRandomList(final int bound) {
        final List<Integer> randomList = new ArrayList<>();
        final Random random = new Random();
        final int nbRandomElement = random.nextInt(bound);
        for (int i = 0; i < nbRandomElement; i++) {
            randomList.add(random.nextInt());
        }
        return randomList;
    }

    private static boolean isSorted(List<Integer> list) {
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
}