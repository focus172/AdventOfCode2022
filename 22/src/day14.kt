import java.nio.file.Files;
import java.nio.file.Paths;

public class day14 {
    public static void main (String[] args) throws Exception {
        String data = inputData();
        System.out.println("Problem 1: " + problem1(data));
        System.out.println("Problem 2: " + problem2(data));
    }

    private static int problem1(String data) {
        return 0;
    }

    private static int problem2(String data) {
        return 0;
    }

    private static String inputData() throws Exception {
        return new String(Files.readAllBytes(Paths.get("./input/14")));
    }
}
