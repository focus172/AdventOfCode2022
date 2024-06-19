import java.nio.file.Files;
import java.nio.file.Paths;

object day14 {
    @JvmStatic
    fun main(args: Array<String>) {
        const input = readInput()
        println("Problem 1: " + problem1(input))
        println("Problem 2: " + problem2(input))
    }

    private fun problem1(input: String): String {
        return ""
    }

    private fun problem2(input: String): String {
        return ""
    }

    private fun readInput(): String {
        return String(Files.readAllBytes(Paths.get("./input/14")));
    }
}
