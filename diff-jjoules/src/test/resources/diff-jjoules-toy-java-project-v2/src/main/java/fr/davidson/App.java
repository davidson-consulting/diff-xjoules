package fr.davidson;

import java.util.*;

/**
 * Hello world!
 */
public class App {

    public static void sort(List<Integer> elementsToSort) {
        Collections.sort(elementsToSort);
    }

    // source: https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Java
    public static List<Integer> quickSort(List<Integer> arr) {
        if (arr.isEmpty())
            return arr;
        else {
            Integer pivot = arr.get(0);

            List<Integer> less = new LinkedList<>();
            List<Integer> pivotList = new LinkedList<>();
            List<Integer> more = new LinkedList<>();

            // Partition
            for (Integer i: arr) {
                if (i.compareTo(pivot) < 0)
                    less.add(i);
                else if (i.compareTo(pivot) > 0)
                    more.add(i);
                else
                    pivotList.add(i);
            }

            // Recursively sort sublists
            less = quickSort(less);
            more = quickSort(more);

            // Concatenate results
            less.addAll(pivotList);
            less.addAll(more);
            return less;
        }
    }

    public void addedStatement() {
        System.out.println("");
        System.out.println("Added");
    }

    public void removedStatement() {
        System.out.println("");
    }

    public void updatedStatement() {
        System.out.println("Updated");
    }

    public void addedAndRemovedStatement() {
        System.out.println("");
        System.out.println("");
        System.out.println("");
        System.out.println("Added");
    }

    public void notExecutedByTestAdded() {
        System.out.println("");
        System.out.println("Added");
    }

    public void notExecutedByTestRemoved() {
        System.out.println("");
    }

    public void notExecutedByTestUpdated() {
        System.out.println("Updated");
    }

    public void notExecutedByTestAddedAndRemoved() {
        System.out.println("");
        System.out.println("");
        System.out.println("");
        System.out.println("Added");
    }

}