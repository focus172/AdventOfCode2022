import java.util.TreeSet;
import java.nio.file.Files;
import java.nio.file.Paths;

class Day1 {
    public static void main(String[] args) {

        //yes I know I input the data twice and that is very unoptimal but i don't care
        System.out.println("Problem 1: " + problem1());
        System.out.println("Problem 2: " + problem2());
    }

    private static int problem1() {
        String data = rawData();
        String[] elfData = data.split("\n\n");

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

    private static int problem2() {
        String data = rawData();
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

    private static String rawData() {
        return new String(Files.readAllBytes(Paths.get("/home/focus/code/aoc/2022/inputs/01")));
    }
}
