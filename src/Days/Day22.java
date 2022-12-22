package Days;

import Helpers.Day22Maze;
import Helpers.FileUtils;

public class Day22 {
    public static void main (String [] args) {
        String raw = rawData();
        String[] split = splitData(raw);
        String board = split[0];
        String guide = split[1];
        String[] boardRows = board.split("\n");

        System.out.println("Problem 1: " + problem1(boardRows, guide));
        System.out.println("Problem 2: " + problem2());
    }

    private static int problem1(String[] boardRows, String guide) {
        Day22Maze maze = new Day22Maze(boardRows);

        String guideCopy = guide; //help with bullshit java stuff
        String currentCommand = "";
        boolean lookingForNumber = true;

        while(guideCopy.length() > 1) {
            currentCommand += guideCopy.substring(0, 1);
            guideCopy = guideCopy.substring(1);

            if (lookingForNumber) {
                try {
                    Integer.parseInt(guideCopy.substring(0, 1));
                } catch (Exception e) {
                    maze.move(currentCommand);
                    currentCommand = "";
                    lookingForNumber = false;
                }
            } else {
                maze.rotate(currentCommand);
                currentCommand = "";
                lookingForNumber = true;
            }

        }

        return (maze.x+1)*1000 + (maze.y+1)*4 + maze.facing;
    }

    private static Object problem2() {
        return null;
    }

    private static String[] splitData(String data) {
        return data.split("\n\n");
    }

    private static String rawData() {
        return FileUtils.fileToString("src/Inputs/Day22Input.txt");
    }
}
