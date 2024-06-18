import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Collections;

public class day11 {
    public static void main(String[] args) throws Exception {
        String data = inputData();
        System.out.println("Problem 1: " + problem1(data)); // A: 57348 (236, 243)
        // too low 851487268 (118307, 116108)
        System.out.println("Problem 2: " + problem2(data));

    }

    private static String problem1(String data) throws Exception {
        ArrayList<MonkeyItem> mkitems = new ArrayList<>();
        ArrayList<Monkey>     monkeys = new ArrayList<>();

        for (String chunk : data.split("\n\n")) {
            Monkey m = new Monkey(chunk, mkitems);
            monkeys.add(m);
        }

        for (int i = 0; i < 20; i++) {
            for (Monkey monkey : monkeys) {
                updateMonkey1(monkey, mkitems);
            }
        }

        //for (Monkey monkey : monkeys) {
        //    System.out.println("Monkey " + monkey.id + ": " + monkey.inspections_count + " inspections.");
        //}

        monkeys.sort((l, r) -> {
            // this comparison is in reverse to sort from high to low
            return Integer.compare(r.inspections_count, l.inspections_count);
        });

        int c1 = monkeys.get(0).inspections_count;
        int c2 = monkeys.get(1).inspections_count;

        return String.format("%d (%d, %d)", c1 * c2, c1, c2); 
    }

    private static int problem2(String data) {
        return 0;
    }

    public static void updateMonkey1(Monkey m, ArrayList<MonkeyItem> items) {
        for (MonkeyItem item : items) {
            if (item.owner != m.id) continue;

            m.inspections_count++;

            item.cncrn = m.change.update(item.cncrn);
            item.cncrn /= 3;
            if (item.cncrn % m.test == 0) {
                item.owner = m.throw_t;
            } else {
                item.owner = m.throw_f;
            }
        }
    }

    public static void updateMonkey2(Monkey m, ArrayList<MonkeyItem> items) {
        for (MonkeyItem item : items) {
            if (item.owner != m.id) continue;

            m.inspections_count++;

            item.cncrn = m.change.update(item.cncrn);
            // item.cncrn /= 3;
            if (item.cncrn % m.test == 0) {
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
    public int cncrn;

    MonkeyItem(int owner, int concern) {
        this.owner = owner;
        this.cncrn = concern;
    }
}

interface MonkeyConcernLambda {
    int update(int c);
}

class Monkey {
    int id;
    MonkeyConcernLambda change;
    int test; // number to check if divisable
    int throw_t;
    int throw_f;

    int inspections_count = 0;

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
        // System.out.println("Got id: " + id);

        // parse items
        for (String item : lines[1].substring(17).split(",")) {
            items.add(new MonkeyItem(this.id, Integer.parseInt(item.substring(1))));
        }

        // HACK parse function
        switch (lines[2].substring(19)) {
            case "old * old" -> this.change = (c) -> { return c * c; };
            case "old * 19" -> this.change = (c) -> { return c * 19; };
            case "old * 13" -> this.change = (c) -> { return c * 13; };
            case "old + 7" -> this.change = (c) -> { return c + 7; };
            case "old + 5" -> this.change = (c) -> { return c + 5; };
            case "old + 4" -> this.change = (c) -> { return c + 4; };
            case "old + 3" -> this.change = (c) -> { return c + 3; };
            case "old + 1" -> this.change = (c) -> { return c + 1; };
            default -> {
                System.out.println("Unhandled case: " + lines[2].substring(19));
                this.change = (c) -> {return c;};
            }
        };

        // parse test
        this.test = Integer.parseInt(lines[3].substring(21));
        // System.out.println("Found test: "+ test);

        // parse true throw
        this.throw_t = Integer.parseInt(lines[4].substring(29));
        // System.out.println("Found true throw: "+ throw_t);

        // parse false throw
        this.throw_f = Integer.parseInt(lines[5].substring(30));
        // System.out.println("Found false throw: "+ throw_f);
    }
}
