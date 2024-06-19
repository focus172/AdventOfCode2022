import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Collections;
import java.util.function.UnaryOperator;

public class day11 {
    public static void main(String[] args) throws Exception {
        String data = inputData();
        System.out.println("Problem 1: " + problem1(data)); // A: 57348 (236, 243)
        System.out.println("Problem 2: " + problem2(data)); // A: 14106266886 (121166, 116421)
    }

    private static String problem1(String data) throws Exception {
        return problemSolver(data, 20, true);
    }

    private static String problem2(String data) throws Exception {
        return problemSolver(data, 10000, false);
    }

    private static String problemSolver(String data, int iterations, boolean concernLowers) throws Exception {
        ArrayList<MonkeyItem> mkitems = new ArrayList<>();
        ArrayList<Monkey>     monkeys = new ArrayList<>();

        for (String chunk : data.split("\n\n")) {
            Monkey m = new Monkey(chunk, mkitems);
            monkeys.add(m);
        }

        long gcd = 1;
        for (Monkey monkey : monkeys) {
            gcd *= monkey.test;
        }

        for (int i = 0; i < iterations; i++) {
            for (Monkey monkey : monkeys) {
                updateMonkey(monkey, gcd, concernLowers, mkitems);
            }
        }

        monkeys.sort((l, r) -> {
            // this comparison is in reverse to sort from high to low
            return Long.compare(r.inspectionCount, l.inspectionCount);
        });

        long c1 = monkeys.get(0).inspectionCount;
        long c2 = monkeys.get(1).inspectionCount;

        return String.format("%d (%d, %d)", c1 * c2, c1, c2); 
    }

    public static void updateMonkey(Monkey m, long gcd, boolean concernLowers, ArrayList<MonkeyItem> items) {
        for (MonkeyItem item : items) {
            if (item.owner != m.id) continue;

            m.inspectionCount++;

            item.concern = m.change.apply(item.concern);

            if (concernLowers) item.concern /= 3;

            // this `should` have no effect on any checks
            item.concern = item.concern % gcd;

            if (item.concern % m.test == 0) {
                item.owner = m.throw_t;
            } else {
                item.owner = m.throw_f;
            }
        }
    }

    private static String inputData() throws Exception {
        return new String(Files.readAllBytes(Paths.get("./input/11")));
    }
}

class MonkeyItem {
    public int owner;
    public long concern;

    MonkeyItem(int owner, long concern) {
        this.owner = owner;
        this.concern = concern;
    }
}

class Monkey {
    int id;
    UnaryOperator<Long> change;
    int test; // number to check if divisable
    int throw_t;
    int throw_f;

    long inspectionCount = 0;

    /// ```monkey
    /// Monkey 0:
    ///   Starting items: 91, 58, 52, 69, 95, 54
    ///   Operation: new = old * 13
    ///   Test: divisible by 7
    ///     If true: throw to monkey 1
    ///     If false: throw to monkey 5
    /// ```
    public Monkey(String chunk, ArrayList<MonkeyItem> items) throws Exception {
        String[] lines = chunk.split("\n");
        if (lines.length < 6) throw new Exception();

        // parsing id
        this.id = Integer.parseInt(lines[0].substring(7, 8));

        // parse items
        for (String item : lines[1].substring(17).split(",")) {
            items.add(new MonkeyItem(this.id, Integer.parseInt(item.substring(1))));
        }

        // HACK: parse function
        this.change = switch (lines[2].substring(19)) {
            case "old * old" -> (c) -> { return c * c; };
            case "old * 19" -> (c) -> { return c * 19; };
            case "old * 13" -> (c) -> { return c * 13; };
            case "old + 7" -> (c) -> { return c + 7; };
            case "old + 5" -> (c) -> { return c + 5; };
            case "old + 4" -> (c) -> { return c + 4; };
            case "old + 3" -> (c) -> { return c + 3; };
            case "old + 1" -> (c) -> { return c + 1; };
            default -> {
                System.err.println("Unhandled function case: " + lines[2].substring(19));
                throw new Exception();
            }
        };

        // parse test
        this.test = Integer.parseInt(lines[3].substring(21));

        // parse true throw
        this.throw_t = Integer.parseInt(lines[4].substring(29));

        // parse false throw
        this.throw_f = Integer.parseInt(lines[5].substring(30));
    }
}
