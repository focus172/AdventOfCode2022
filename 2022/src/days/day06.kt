import java.io.File
package build;

object Day6 {
    @JvmStatic
    fun main(args: Array<String>) {
        val raw = rawData()
        println("Problem 1: " + problem1(raw))
        println("Problem 2: " + problem2(raw))
    }

    private fun problem1(rawData: String): Int {
        return problemSolver(rawData, 4)
    }

    private fun problem2(rawData: String): Int {
        return problemSolver(rawData, 14)
    }

    private fun problemSolver(rawData: String, depth: Int): Int {
        for (i in 0..rawData.length) { // this will go out of bounds sometimes
            if (isPacket(rawData.substring(i, i + depth))) return i + depth
        }
        return -1
    }

    private fun isPacket(charStream: String): Boolean {
        val letterMap: HashMap<Char, Boolean> = HashMap()
        for (letter in charStream) {
            if (letterMap.contains(letter)) {
                return false
            }
            letterMap.put(letter, true)
        }
        return true
    }

    private fun rawData(): String {
        return File("./input/06").useLines {
            it.elementAt(0)
        }
    }
}
