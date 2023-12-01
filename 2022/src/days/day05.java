import java.nio.file.Files;
import java.nio.file.Paths;

import java.util.ArrayList;
import java.util.Stack;


public class day05 {
    public static void main (String [] args) {
        String[] splitData = rawData().split("\n\n");
        String blockData = splitData[0];
        String moveData = splitData[1];

        ArrayList<Stack<Character>> blockStacks = getStackData(blockData);
        int[][] blockMovements = getMoveData(moveData);

        /*
        for(Stack<Character> stack : blockStacks) {
            System.out.println("Stack: " + stack.toString());
        }
        for (int i = 0; i < 10; i++) {
            System.out.println("Movement: " + blockMovements[i][0] + " " + blockMovements[i][1]  + " " + blockMovements[i][2] );
        }
         */

        System.out.println("Problem 1: " + problem1(blockStacks, blockMovements));
        System.out.println("Problem 2: " + problem2(getStackData(blockData), blockMovements));
    }

    private static String problem1(ArrayList<Stack<Character>> blockStacks, int[][] blockMovements) {
        for (int[] movementDesc: blockMovements) {
            for (int i = 0; i < movementDesc[0]; i++) {
                Character removed = blockStacks.get(movementDesc[1]-1).pop();
                Stack<Character> temp = blockStacks.get(movementDesc[2]-1);
                temp.add(removed);
                blockStacks.set(movementDesc[2]-1, temp);
            }
        }

        StringBuilder retStr = new StringBuilder();
        for (Stack<Character> stack : blockStacks) { retStr.append(stack.peek()); }

        return retStr.toString();
    }

    private static String problem2(ArrayList<Stack<Character>> blockStacks, int[][] blockMovements) {
        for (int[] movementDesc: blockMovements) {
            Stack<Character> crateStorage = new Stack<>();
            for (int i = 0; i < movementDesc[0]; i++) {
                crateStorage.add(blockStacks.get(movementDesc[1]-1).pop());
            }

            Stack<Character> temp = blockStacks.get(movementDesc[2]-1);
            while (!crateStorage.isEmpty()) {
                temp.add(crateStorage.pop());
            }
            blockStacks.set(movementDesc[2]-1, temp);
        }

        StringBuilder retStr = new StringBuilder();
        for (Stack<Character> stack : blockStacks) { retStr.append(stack.peek()); }
        return retStr.toString();
    }

    private static ArrayList<Stack<Character>> getStackData(String stackData) {
        // each block is 4 characters long
        // however the last one is only 3
        // when there are no more blocks in a line it stops listing it

        String[] stackLines = stackData.split("\n");
        ArrayList<Stack<Character>> retStacks = new ArrayList<>();
        for(int i = 0; i < 9; i++) { // try to derive this number from the data
            retStacks.add(new Stack<>());
        }

        for (int line = stackLines.length-1; line >= 0; line--) {
            String stackLine = stackLines[line];
            for (int i = 0; i < stackLine.length(); i+=4) {
                Character boxedValue = parseBox(stackLine.substring(i, i+2));
                if (boxedValue != null) {
                    Stack<Character> temp = retStacks.get(getRow(i));
                    temp.add(boxedValue);
                    //System.out.println(temp.toString());
                    retStacks.set(getRow(i), temp);
                }
            }
        }

        return retStacks;
    }

    private static int getRow(int index) {
        return (index/4);
    }

    private static Character parseBox(String boxValue) {
        int beginBox = boxValue.indexOf('[');
        if (beginBox < 0) {return null;}
        return boxValue.charAt(beginBox + 1);
    }

    private static int[][] getMoveData(String moveData) {
        String[] moves = moveData.split("\n");
        int[][] returnedMoves = new int[moves.length][3];
        for (int i = 0; i < moves.length; i++) {
            returnedMoves[i] = parseMovement(moves[i]);
        }

        return returnedMoves;
    }

    private static int[] parseMovement(String movementDesc) {
        try {
            String[] dataDump = movementDesc.split(" ");
            int[] retArr = new int[3];
            retArr[0] = Integer.parseInt(dataDump[1]);
            retArr[1] = Integer.parseInt(dataDump[3]);
            retArr[2] = Integer.parseInt(dataDump[5]);
            return retArr;
        } catch (Exception e) {
            e.printStackTrace();
            System.out.println("Bad data format exception");
            return null;
        }
    }

    private static String rawData() {
        try {
            return new String(Files.readAllBytes(Paths.get("./input/05")));
        } catch (Exception e) {
            return null;
        }
    }
}
