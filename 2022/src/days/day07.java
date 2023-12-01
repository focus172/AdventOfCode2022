import java.util.ArrayList;
import java.nio.file.Files;
import java.nio.file.Paths;

public class day07 {
    String input;
    TreeObject root;

    public static void main (String [] args) throws Exception {
        day07 runner = new day07();

        System.out.println("Problem 1: " + runner.p1());
        System.out.println("Problem 2: " + runner.p2());
    }

    day07() throws Exception {
        this.input = get_input();
        String[] commandLines = this.input.split("\n");
        this.root = makeFileSystem(commandLines);
    }

    private int p1() {
        return this.p1_rec(this.root);
        // return out.to_string();
        // return new String(out);
        // recurse through the tree to find all the directories
        // return recurseProblem1(this.root);
    }

    private int p2() {
        String[] commandLines = this.input.split("\n");

        TreeObject rootDirPointer = makeFileSystem(commandLines);
        final int amountToDelete = 3441553;

        return recurseProblem2(rootDirPointer, amountToDelete).size;
    }

    private static int p1_rec(TreeObject curDur) {
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
            for (TreeObject child : curDur.children) {
                sum += p1_rec(child);
            }
            return sum;
        }
    }

    private static TreeObject recurseProblem2(TreeObject curDur, int amountToDelete) {
        // we want to find the smallest directory that exceeds amount to delete
        // to do this the base case is if there is no reason to go further
        // a result of null means there were no canidits found


        if (curDur.size < amountToDelete || !curDur.isDir) {
            return null;
        } else {
            TreeObject bestFound = curDur;
            for (TreeObject child : curDur.children) {
                TreeObject candidate = recurseProblem2(child, amountToDelete);
                if (candidate != null && candidate.size < bestFound.size) {
                    bestFound = candidate;
                }
            }
            return bestFound;
        }

    }


    private static TreeObject makeFileSystem(String[] commandLines) {
        TreeObject rootDirPointer = new TreeObject("", null, true, 0);
        TreeObject curDir = rootDirPointer;
        for (String line : commandLines) {
            if (line.startsWith("$")) {
                String command = line.substring(2, 4);
                if (command.equals("cd")) {
                    String argument = line.substring(5);
                    if (argument.equals("..")) {
                        curDir = curDir.parent;
                    } else {
                        TreeObject newDir = new TreeObject(argument, curDir, true, 0);
                        curDir.addChild(newDir);
                        curDir = newDir;
                    }
                } else if (command.equals("ls")) {
                    continue;
                } else {
                    System.out.println("Bad command");
                    return null;
                }
            } else {
                if (line.startsWith("dir")) {
                    ;
                } else {
                    String[] arguments = line.split(" ");
                    int size = Integer.parseInt(arguments[0]);
                    TreeObject newFile = new TreeObject(arguments[1], curDir, false, size);
                    curDir.addChild(newFile);
                }
            }
        }
        rootDirPointer.getSize(); //this will update all the sizes of the directories
        return rootDirPointer;
    }


    private static String get_input() throws Exception {
        return new String(Files.readAllBytes(Paths.get("./input/07")));
    }
}

class TreeObject {
    public String name;
    public TreeObject parent;
    public boolean isDir;
    public ArrayList<TreeObject> children;

    public int size; // this includes all the children

    public TreeObject (String name, TreeObject parent, Boolean isDir, int size) {
        this.name = name;
        this.parent = parent;
        this.isDir = isDir;
        this.size = size;

        children = new ArrayList<>();
    }

    public boolean addChild(TreeObject child) {
        children.add(child);
        return true;
    }

    public int getSize() {
        if (isDir) {
            int sum = 0;
            for (TreeObject child: children) {
                sum += child.getSize();
            }
            size = sum;
            return size;
        } else {
            return size;
        }

    }
}

// class Result<T, E> {
//     public T ok;
//     public E err;
//
//     public Result(T ok) {
//         this.ok = ok;
//         this.err = null;
//     }
//
//     public T unwrap() throws Exception {
//         if (this.err != null) {
//             throw new Exception("Unwrapped error: " + this.err);
//         }
//         return this.ok;
//     }
//
//     public T unwrap_or(T alt) {
//         if (this.ok != null) {
//             this.ok
//         } else {
//             alt
//         }
//     }
// }
