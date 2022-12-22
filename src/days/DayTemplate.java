package days;

import helpers.FileUtils;

public class DayTemplate {
    public static void main (String [] args) {
        System.out.println("Problem 1: " + problem1());
        System.out.println("Problem 2: " + problem2());
    }

    private static Object problem1() {
        return null;
    }

    private static Object problem2() {
        return null;
    }

    private static String rawData() {
        return FileUtils.fileToString("src/Inputs/DayXInput.txt");
    }
}
