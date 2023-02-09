package days

import helpers.FileUtils
import java.awt.Point

object Day09 {
    @JvmStatic
    fun main(args: Array<String>) {
        val raw = rawData()
        val lines = raw.split("\n")
        println("Problem 1: " + problem1(lines))
        println("Problem 2: " + problem2())
    }

    private fun problem1(lines: List<String>): Int {
        val visitedMap = HashSet<Point>()
        val curHeadPos = Point(0, 0)
        val curTailPos = Point(0, 0)
        for (line in lines) {
            val split = line.split(" ")
            val direction: Point = parseDirection(split[0])
            val distance: Int = Integer.parseInt(split[1])

            for (i in 1 .. distance) {
                curHeadPos.translate(direction.x, direction.y)
                if (curHeadPos.distance(curTailPos) >= 2.0) { curTailPos.setLocation(
                    curHeadPos.x-direction.x, curHeadPos.y-direction.y
                ); }
                visitedMap.add(curTailPos)
            }
        }
        return visitedMap.size
    }

    private fun problem2(): Int {
        return 0
    }

    private fun parseDirection(letter: String): Point {
        return when (letter) {
            "U" -> { Point(0, 1) }
            "D" -> { Point(0, 1) }
            "R" -> { Point(1, 0) }
            "L" -> { Point(-1, 0 ) }
            else -> { Point(0, 0) }
        }
    }

    private fun rawData(): String {
        return FileUtils.fileToString("src/inputs/Day09Input.txt")
    }
}