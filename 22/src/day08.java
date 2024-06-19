import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.Map;
import java.util.Map;

class day08 {
    public static void main(String[] args) throws Exception {
        String data = readInput();
        System.out.println("Day 8");
        System.out.println("Problem 1: " + problem1(data));
        System.out.println("Problem 2: " + problem2(data));
    }

    private static String problem1(String data) {
        HashSet<Map.Entry<Integer, Integer>> positions = new HashSet<>();

        char[][] forest = data.split("\n").stream().map( line -> line.getBytes() );

        // from the left
        for (int r = 0; r < forest.length; r++) {
            char[] row = forest[r];

            char max = 0;
            for (int c = 0; c < row.length; c++) {
                char tree = row[c];

                // comparing the ascii values of numbers is the equivilent to
                // comparing their numerical value.
                if (tree > max) {
                    positions.add(Map.entry(r, c));
                    max = tree;
                }
            }
        }

        // from the right
        //for (int r = 0; r < lines.length; r++) {
        //    String line = lines[r];
        //
        //    char max = 0;
        //    for (int c = line.length() - 1; c >= 0; c--) {
        //        char tree = line.charAt(c);
        //
        //        if (tree > max) {
        //            positions.add(Map.entry(r, c));
        //            max = tree;
        //        }
        //    }
        //}


//    // from the top
//    for (let c = 0; c < dataLines[0].length; c++) {
//        var maxFound = -1;
//        for (let r = 0; r < dataLines.length; r++) {
//            let line = dataLines[r];
//            let value = line[c];
//            if (value > maxFound) {
//                positionMap.set(r+", "+c, true)
//                maxFound = value
//            }
//        }
//    }

//    for (let c = 0; c < dataLines[0].length; c++) {
//        var maxFound = -1;
//        for (let r = dataLines.length-1; r >= 0 ; r--) {
//            let line = dataLines[r];
//            let value = line[c];
//            if (value > maxFound) {
//                positionMap.set(r+", "+c, true)
//                maxFound = value
//            }
//        }
//    }
//
        return "" + positions.size();
    }

    private static String problem2(String data) {
        return "";
    }

    private static String readInput() throws Exception {
        return new String(Files.readAllBytes(Paths.get("./input/08")));
    }
}

//function problem2(dataLines) {
//    var maxFound = 0;
//
//    for (let r = 0; r < dataLines.length; r++) {
//        var line = dataLines[r]
//        for (let c = 0; c < line.length; c++) {
//            var value = line[c]
//
//            var fromTop = getFromTop(r, c, dataLines, value)
//            var fromBottom = getFromBottom(r, c, dataLines, value)
//            var fromLeft = getFromLeft(r, c, dataLines, value)
//            var fromRight = getFromRight(r, c, dataLines, value)
//
//            var score = fromBottom * fromLeft * fromTop * fromRight;
//
//            if (score > maxFound) {
//                maxFound = score
//            }
//        }
//    }
//
//    return maxFound;
//}
//
//function getFromTop(row, col, dataArray, value) {
//    var distance = 1
//    var loop = true
//    while (loop) {
//        if (row - distance < 0) { loop = false; distance -= 1;}
//        else if (dataArray[row-distance][col] >= value) { loop = false; }
//        else { distance += 1; }
//    }
//    return distance
//}
//
//function getFromBottom(row, col, dataArray, value) {
//    var distance = 1
//    var loop = true
//    while (loop) {
//        if (row + distance >= dataArray.length) { loop = false; distance -= 1;}
//        else if (dataArray[row+distance][col] >= value) { loop = false; }
//        else { distance += 1; }
//    }
//    return distance
//}
//
//function getFromRight(row, col, dataArray, value) {
//    var distance = 1
//    var loop = true
//    while (loop) {
//        if (col + distance >= dataArray[row].length) { loop = false; distance -= 1;}
//        else if (dataArray[row][col+distance] >= value) { loop = false; }
//        else { distance += 1; }
//    }
//    return distance
//}
//
//function getFromLeft(row, col, dataArray, value) {
//    var distance = 1
//    var loop = true
//    while (loop) {
//        if (col - distance < 0) { loop = false; distance -= 1; }
//        else if (dataArray[row][col-distance] >= value) { loop = false; }
//        else { distance += 1; }
//    }
//    return distance
//}
