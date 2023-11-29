package days;

import helpers.Day7TreeObject;
import helpers.FileUtils;

public class Day7 {
    public static void main (String [] args) {
        String raw = rawData();
        String[] lines = outputLines(raw);

        System.out.println("Problem 1: " + problem1(lines));
        System.out.println("Problem 2: " + problem2(lines));
    }

    private static int problem1(String[] commandLines) {
        // read the data into a tree of objects
        Day7TreeObject rootDirPointer = makeFileSystem(commandLines);

        // recurse through the tree to find all the directories
        return recurseProblem1(rootDirPointer);
    }

    private static int problem2(String[] commandLines) {
        Day7TreeObject rootDirPointer = makeFileSystem(commandLines);
        final int amountToDelete = 3441553;

        return recurseProblem2(rootDirPointer, amountToDelete).size;
    }

    private static int recurseProblem1(Day7TreeObject curDur) {
        // In each directory
        // > check if its size meets the threshold
        // > call on each if children
        if (!curDur.isDir) {
            return 0;
        } else {
            int sum = 0;
            if (curDur.size < 100000) {
                sum += curDur.size;
            }
            for (Day7TreeObject child : curDur.children) {
                sum += recurseProblem1(child);
            }
            return sum;
        }
    }

    private static Day7TreeObject recurseProblem2(Day7TreeObject curDur, int amountToDelete) {
        // we want to find the smallest directory that exceeds amount to delete
        // to do this the base case is if there is no reason to go further
        // a result of null means there were no canidits found


        if (curDur.size < amountToDelete || !curDur.isDir) {
            return null;
        } else {
            Day7TreeObject bestFound = curDur;
            for (Day7TreeObject child : curDur.children) {
                Day7TreeObject candidate = recurseProblem2(child, amountToDelete);
                if (candidate != null && candidate.size < bestFound.size) {
                    bestFound = candidate;
                }
            }
            return bestFound;
        }

    }


    private static Day7TreeObject makeFileSystem(String[] commandLines) {
        Day7TreeObject rootDirPointer = new Day7TreeObject("", null, true, 0);
        Day7TreeObject curDir = rootDirPointer;
        for (String line : commandLines) {
            if (line.startsWith("$")) {
                String command = line.substring(2, 4);
                if (command.equals("cd")) {
                    String argument = line.substring(5);
                    if (argument.equals("..")) {
                        curDir = curDir.parent;
                    } else {
                        Day7TreeObject newDir = new Day7TreeObject(argument, curDir, true, 0);
                        curDir.addChild(newDir);
                        curDir = newDir;
                    }
                } else if (command.equals("ls")) {
                    ;
                } else {
                    System.out.println("Bad command");
                }
            } else {
                if (line.startsWith("dir")) {
                    ;
                } else {
                    String[] arguments = line.split(" ");
                    int size = Integer.parseInt(arguments[0]);
                    Day7TreeObject newFile = new Day7TreeObject(arguments[1], curDir, false, size);
                    curDir.addChild(newFile);
                }
            }
        }
        rootDirPointer.getSize(); //this will update all the sizes of the directories
        return rootDirPointer;
    }

    private static String[] outputLines(String rawData) {
        return rawData.split("\n");
    }

    private static String rawData() {
        return FileUtils.fileToString("src/Inputs/Day7Input.txt");
    }
}

import java.util.ArrayList;

public class Day7TreeObject {

    public String name;
    public Day7TreeObject parent;
    public boolean isDir;
    public ArrayList<Day7TreeObject> children;

    public int size; // this includes all the children

    public Day7TreeObject (String name, Day7TreeObject parent, Boolean isDir, int size) {
        this.name = name;
        this.parent = parent;
        this.isDir = isDir;
        this.size = size;

        children = new ArrayList<>();
    }

    public boolean addChild(Day7TreeObject child) {
        children.add(child);
        return true;
    }

    public int getSize() {
        if (isDir) {
            int sum = 0;
            for (Day7TreeObject child: children) {
                sum += child.getSize();
            }
            size = sum;
            return size;
        } else {
            return size;
        }

    }

}
