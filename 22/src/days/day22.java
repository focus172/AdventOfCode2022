package days;

import helpers.Day22Maze;
import helpers.FileUtils;

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

public class Day22Maze {

    public int facing = 0; //Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^)
    public int y;
    public int x;
    public int[][] board;
    String[] boardRows;

    public Day22Maze(String[] boardRows) {
        this.boardRows = boardRows;
        this.board = getRepersentation(boardRows);
        this.y = 0;
        this.x = firstAvaliableX(y);
        //printMaze();
    }

    /*
    private void printMaze() {
        for (int r = 0; r < board.length; r++) {
            for (int c = 0; c < board[r].length; c++) {
                if (board[r][c] == 1) {System.out.print(".");}
                else if (board[r][c] == -1) {System.out.print("#");}
                else {System.out.print(" ");}
            }
            System.out.println();
        }
    }
     */


    public void move(String command) {
        for (int i = 0; i < Integer.parseInt(command); i++) {
            System.out.println(x + ", " + y);
            if (facing == 0) {
                int nextX = x+1;

                if ( nextX >= board[y].length || board[y][nextX] == 0 ) {
                    if (board[y][firstAvaliableX(y)] != -1) { x = firstAvaliableX(y); }
                    else { return; }
                }
                else if (board[y][nextX] == -1) { return; }
                else if (board[y][nextX] == 1 ){ x = nextX;}
                else { System.out.println("Something went wrong getting movement to: " + nextX); }
            } else if (facing == 1) {
                int nextY = y+1;

                if ( nextY >= board.length || board[nextY][x] == 0 ) {
                    if (board[firstAvaliableY(x)][x] != -1) { y = firstAvaliableY(x); }
                    else { return; }
                }
                else if (board[nextY][x] == -1) { return; }
                else if (board[nextY][x] == 1 ){ y = nextY; }
                else { System.out.println("Something went wrong getting movement to: " + nextY); }
            } else if (facing == 2) {
                int nextX = x-1;

                if ( nextX < 0 || board[y][nextX] == 0 ) {
                   if (board[y][lastAvaliableX(y)] != -1) { x = lastAvaliableX(y); }
                   else { return; }
                }
                else if (board[y][nextX] == -1) { return; }
                else if (board[y][nextX] == 1 ){ x = nextX; }
                else { System.out.println("Something went wrong getting movement to: " + nextX); }
            } else if (facing == 3) {
                int nextY = y-1;

                if ( nextY < 0 || board[nextY][x] == 0 ) {
                    if (board[lastAvaliableY(x)][x] != -1) { y = lastAvaliableY(x); }
                    else { return; }
                }
                else if (board[nextY][x] == -1) { return; }
                else if (board[nextY][x] == 1 ){ y = nextY; }
                else { System.out.println("Something went wrong getting movement to: " + nextY); }
            }

            else { System.out.println("Something went wrong moving in direction: " + facing); }
        }
    }

    public void rotate(String command) {
        if (command.equals("R")) { facing++; }
        else if (command.equals("L")) { facing--; }
        else {System.out.println("invalid turn command: \"" + command + "\"");}

        if (facing == 4) { facing = 0;}
        else if (facing == -1) { facing = 3;}
    }

    private int firstAvaliableX(int row) {
        for (int col = 0; col < board.length; col++) {
            if (board[row][col] == 1) {
                return col;
            }
        }
        return -1;
    }

    private int firstAvaliableY(int col) {
        for (int row = 0; row < board.length; row++) {
            if (board[row][col] == 1) {
                return row;
            }
        }
        return -1;
    }

    private int lastAvaliableX(int row) {
        for (int col = board[row].length-1; col >= 0 ; col--) {
            if (board[row][col] == 1) {
                return col;
            }
        }
        return -1;
    }

    private int lastAvaliableY(int col) {
        for (int row = board.length-1; row >= 0; row--) {
            if (board[row][col] == 1) {
                return row;
            }
        }
        return -1;
    }

    private int[][] getRepersentation(String[] boardRows) {
        //I know that the first row is the longest so im going to cheat
        int[][] retArr = new int[boardRows.length][boardRows[0].length()];
        for (int r = 0; r < boardRows.length; r++) {
            for (int c = 0; c < boardRows[r].length(); c++) {
                //0 = blank/empty
                //1 = walkable terrain
                //-1 = wall
                char curLetter = boardRows[r].charAt(c);
                if (curLetter == '.') {retArr[r][c] = 1;}
                else if (curLetter == '#') {retArr[r][c] = -1;}
            }
        }
        return retArr;
    }

}
