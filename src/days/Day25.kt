package days

import helpers.FileUtils
import helpers.Day25SNAFUNumber

object Day25 {
    @JvmStatic
    fun main(args: Array<String>) {
        val rawData = rawData()
        val dataLines = rawData.split("\n")
        println("Problem 1: " + problem1(dataLines))
    }

    private fun problem1(datelines: List<String>): String {
        var sum: Long = 0
        for (line in datelines) {
            val curSNAFU = Day25SNAFUNumber(line)
            sum += curSNAFU.value
        }
        return Day25SNAFUNumber(sum).representation
    }

    private fun rawData(): String {
        return FileUtils.fileToString("src/inputs/Day25Input.txt")
    }
}