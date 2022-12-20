package Days;

import Helpers.FileUtils;

public class Day2 {
    public static void main (String [] args) {
        char[][] parsedData = parsedData();
        System.out.println("Problem 1: " + problem1(parsedData));
        System.out.println("Problem 2: " + problem2(parsedData));
    }

    private static int problem1(char[][] inputData) {
        int sum = 0;
        for (char[] game : inputData) {
            sum += calculateGame1Score(game);
        }
        return sum;
    }

    private static int problem2(char[][] inputData) {
        int sum = 0;
        for (char[] game : inputData) {
            sum += calculateGame2Score(game);
        }
        return sum;
    }

    private static int calculateGame1Score(char[] moves) {
        //yes I could use a switch but it seems optimization is not important
        if (moves[0] == 'A') { //rock
            if (moves[1] == 'X') { //rock
                return 4;
            } else if (moves[1] == 'Y') { //paper
                return 8;
            } else if (moves[1] == 'Z') { //sharp thing
                return 3;
            } else {
                System.out.println("Looks like there is some bad data here");
                return 0;
            }
        } else if (moves[0] == 'B') { //paper
            if (moves[1] == 'X') { //rock
                return 1;
            } else if (moves[1] == 'Y') { //paper
                return 5;
            } else if (moves[1] == 'Z') { //sharp thing
                return 9;
            } else {
                System.out.println("Looks like there is some bad data here");
                return 0;
            }
        } else if (moves[0] == 'C') { //sharp thing
            if (moves[1] == 'X') { //rock
                return 7;
            } else if (moves[1] == 'Y') { //paper
                return 2;
            } else if (moves[1] == 'Z') { //sharp thing
                return 6;
            } else {
                System.out.println("Looks like there is some bad data here");
                return 0;
            }
        } else {
            System.out.println("Looks like there is some bad data here");
            return 0;
        }
    }

    private static int calculateGame2Score(char[] moves) {
        int score = 0;
        // A = 0, B = 1, C = 2
        // X = 0, Y = 1, Z = 2
        int[][] scoreForWhatToChoose = new int[][]{{3, 1, 2}, {1, 2, 3}, {2, 3, 1}};
        int[] scoreForMatchResult = new int[]{0, 3, 6};
        score += scoreForMatchResult[valueOf(moves[1])];
        score += scoreForWhatToChoose[valueOf(moves[0])][valueOf(moves[1])];
        return score;
    }

    private static int valueOf(char input) {
        // this code is weird in that it serves two functions and for clarity this should be split
        // but im not going to do that
        if (input == 'A' || input == 'X') {return 0;}
        if (input == 'B' || input == 'Y') {return 1;}
        if (input == 'C' || input == 'Z') {return 2;}

        System.out.println("something whent wring");
        return -1;
    }

    private static char[][] parsedData() {
        String raw = rawData();
        String[] rounds = raw.split("\n");

        char[][] retData = new char[rounds.length][2];
        for (int i = 0; i < rounds.length; i++) {
            String[] temp = rounds[i].split(" ");

            retData[i][0] = temp[0].charAt(0);
            retData[i][1] = temp[1].charAt(0);
        }
        return retData;
    }

    private static String rawData() {
        return FileUtils.fileToString("src/Inputs/Day2Input.txt");
    }
}
