// package build;

import java.util.TreeSet;
import java.nio.file.Files;
import java.nio.file.Paths;

public class day01 {
    private String input;

    public day01() {
        this.input = data();
    }

    public static void main(String[] args) {
        day01 runner = new day01();

        System.out.println("Day 1.");
        System.out.println("Problem 1: " + runner.p1());
        System.out.println("Problem 2: " + problem2());
    }

    public int p1() {
        String[] elfData = this.input.split("\n\n");

        int max = 0;
        for (String dataSet : elfData) {
            int temp = 0;
            for (String item : dataSet.split("\n")) {
                temp += Integer.valueOf(item);
            }
            if (temp > max) {
                max = temp;
            }
        }
        return max;
    }

    public static int problem2() {
        String data = data();
        String[] elfData = data.split("\n\n");

        TreeSet<Integer> top3Tree = new TreeSet<Integer>();
        for (String dataSet : elfData) {
            int temp = 0;
            for (String item : dataSet.split("\n")) {
                temp += Integer.valueOf(item);
            }

            top3Tree.add(temp);
            if (top3Tree.size() > 3) {
                top3Tree.pollFirst();
            }
        }

        // debug code
        // System.out.println(top3Tree.toString());

        int retVaue = 0;
        for (Integer best : top3Tree) {
            retVaue += best.intValue();
        }

        return retVaue;
    }

    private static String data() {
        try {
            return new String(Files.readAllBytes(Paths.get("./input/01")));
        } catch (Exception e) {
            System.out.println(e);
            return "";
        }
    }
}
