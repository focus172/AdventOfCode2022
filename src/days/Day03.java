package days;

import helpers.FileUtils;

public class Day3 {
    public static void main (String [] args) {
        String raw = rawData();
        System.out.println("Problem 1: " + problem1(parsedData(raw)));
        System.out.println("Problem 2: " + problem2(part2Data(raw)));
    }

    private static int problem1(String[][] parsedData) {
        int sum = 0;
        for (String[] bag: parsedData) {
            char badCharacter = findDuplicate(bag);
            sum +=scoreOf(badCharacter);
        }
        return sum;
    }

    private static int problem2(String[] inData) {
        int sum = 0;
        for (int set = 0; set < inData.length; set += 3) {
            //char[] quickData = inData[set].toCharArray();
            String elf1 = inData[set];
            String elf2 = inData[set+1];
            String elf3 = inData[set+2];
            boolean shouldCount = true;
            for (char letter : elf1.toCharArray()) {
                String check = String.valueOf(letter);
                if (shouldCount && elf2.contains(check) && elf3.contains(check)) {
                    sum += scoreOf(letter);
                    shouldCount = false;
                }
            }
        }
        return sum;
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
        int temp = input;
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

    private static String[] part2Data(String data) {
        return data.split("\n");
    }

    private static String rawData() {
        return FileUtils.fileToString("src/Inputs/Day3Input.txt");
    }
}
