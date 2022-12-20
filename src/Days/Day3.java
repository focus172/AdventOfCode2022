package Days;

import Helpers.FileUtils;

public class Day3 {
    public static void main (String [] args) {
        /*
        String garbage = "abcdefgxyzABCEDXYZ";
        for (int i = 0; i < garbage.length(); i++) {
            System.out.println(scoreOf(garbage.charAt(i)));
        }
        */
        String[][] parsedData = parsedData(rawData());
        System.out.println("Problem 1: " + problem1(parsedData));
        System.out.println("Problem 2: " + problem2());
    }

    private static int problem1(String[][] parsedData) {
        int sum = 0;
        for (String[] bag: parsedData) {
            char badCharacter = findDuplicate(bag);
            sum +=scoreOf(badCharacter);
        }
        return sum;
    }

    private static int problem2() {

        return 0;
    }

    private static char findDuplicate(String[] input) {
        //if (input.length != 2) {return '_';}
        for (int i = 0; i < input[0].length(); i++) {
            if (input[1].contains( String.valueOf( input[0].charAt(i) ) ) )  {
                return input[0].charAt(i);
            }
        }
        return '_';
    }

    private static int scoreOf(char input) {
        int temp = (int)input;
        temp -= 64;
        if (temp <= 26) { return temp+26;}
        else { return (int)input - 96; }
    }

    private static String[][] parsedData(String data) {
        String[] lines = data.split("\n");
        String[][] out = new String[lines.length][2];
        for (int i = 0; i < lines.length; i++) {
            out[i][0] = lines[i].substring(0,lines[i].length()/2);
            out[i][1] = lines[i].substring(lines[i].length()/2);
        }
        return out;
    }

    private static String rawData() {
        return FileUtils.fileToString("src/Inputs/Day3Input.txt");
    }
}
