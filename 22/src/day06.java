import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;

class day06 {
    public static void main(String[] args) throws Exception {
        String data = input_data();
        System.out.println("Problem 1: " + problem1(data));
        System.out.println("Problem 2: " + problem2(data));
    }

    private static int packet_scanner(String data, int depth) {
        for (int i = 0; i < data.length() - depth; i++) {
            if (isPacket(data.substring(i, i + depth))) {
                return i + depth;
            }
        }
        return -1;
    }

    private static boolean isPacket(String stream) {
        HashSet<Character> letters = new HashSet<Character>();

        // char[] list = new char[stream.length()];

        for (int i = 0; i < stream.length(); i++) {
            char c = stream.charAt(i);
            if (!letters.add(c)) {
                return false;
            }
        }
        return true;
    }


    private static int problem1(String data) {
        return packet_scanner(data, 4);
    }

    private static int problem2(String data) {
        return packet_scanner(data, 14);
    }

    private static String input_data() throws Exception {
        return new String(Files.readAllBytes(Paths.get("./input/06")));
    }
}
