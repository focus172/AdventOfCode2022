import java.nio.file.Files;
import java.nio.file.Paths;

public class day04 {
    public static void main (String [] args) {
        String raw = rawData();
        System.out.println("Problem 1: " + problem1(parseData(raw)));
        System.out.println("Problem 2: " + problem2(parseData(raw)));
    }

    private static int problem1(String[] parsedData) {
        int sum = 0;

        for (String group: parsedData) {
            String[] assignments = group.split(",");
            String elf1 = assignments[0];
            String elf2 = assignments[1];
            if (checkContained(elf1, elf2) || checkContained(elf2, elf1)) {
                sum += 1;
            }
        }
        return sum;
    }

    private static boolean checkContained(String elf1, String elf2) {
        if (Integer.parseInt(elf1.split("-")[0]) >= Integer.parseInt(elf2.split("-")[0])) {
            if (Integer.parseInt(elf1.split("-")[1]) <= Integer.parseInt(elf2.split("-")[1])) {
                return true;
            }
        }
        return false;
    }

    private static int problem2(String[] parsedData) {
        int sum = 0;

        for (String group: parsedData) {
            String[] assignments = group.split(",");
            String elf1 = assignments[0];
            String elf2 = assignments[1];
            if (checkOverlap(elf1, elf2)) {
                sum += 1;
            }
        }
        return sum;
    }

    private static boolean checkOverlap(String elf1, String elf2) {
        //if the lower bound of the second is less than the upper bound of the first
        //or the lower bound of the first is less than the upper bound of the second
        String[] elf1Bounds = elf1.split("-");
        String[] elf2Bounds = elf2.split("-");
        if (Integer.parseInt(elf2Bounds[0]) <= Integer.parseInt(elf1Bounds[1]) && Integer.parseInt(elf1Bounds[0]) <= Integer.parseInt(elf2Bounds[1])) {
            return true;
        }
        return false;
    }

    private static String[] parseData(String data) {
        return data.split("\n");
    }

    private static String rawData() {
        try {
            return new String(Files.readAllBytes(Paths.get("./inputs/04")));
        } catch (Exception e) {
            return null;
        }
    }
}
