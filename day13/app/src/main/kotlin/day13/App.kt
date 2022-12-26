package day13

class App {
    data class Node(
        val children: MutableList<Node> = mutableListOf<Node>(),
        val value: Int = -1,
        val parent: Node? = null
    ) {
        override fun toString(): String {
            if (value > -1) {
                return "$value"
            } 
            return "[${children.map { it.toString() }.joinToString(",")}]"
        }

        fun isLeaf(): Boolean {
            return children.size == 0 && value != -1
        }
    }

    private fun readInFileCalledExamplePerLine(): Pair<List<Node>, List<Node>> {
        val lines = javaClass.getResource("/input").readText().split("\n")
        val lefts = mutableListOf<Node>()
        val rights = mutableListOf<Node>()
        lines.filter { it.length > 0 }
            .forEachIndexed { i, line ->
                val toPut = if (i%2 == 0) { lefts } else { rights }
                val root = Node()
                var temp = root
                var k = 0
                while (k < line.toCharArray().size) {
                    if (line[k].isDigit()) {
                        val nextNonDigitIndex = nextNonDigitIndexFrom(k, line)
                        val value = line.substring(k, nextNonDigitIndex).toInt()
                        val newNode = Node(value = value, parent = temp)
                        temp.children.add(newNode)
                        k = nextNonDigitIndex
                        continue
                    } else if (line[k] == ']') {
                        temp = temp.parent!!
                    } else if (line[k] == '[') {
                        val newNode = Node(parent = temp)
                        temp.children.add(newNode)
                        temp = newNode
                    }
                    k++
                }
                toPut.add(root)
            }
        return Pair(lefts, rights)
    }

    private fun nextNonDigitIndexFrom(i: Int, s: String): Int {
        for (j in i+1..s.toCharArray().size) {
            if (!s[j].isDigit()) {
                return j
            }
        }
        return -1
    }

    private fun solve(lefts: List<Node>, rights: List<Node>) {
        var sum = 0
        lefts.mapIndexed { i, e -> Pair(e, rights[i]) }
            .forEachIndexed { i, it ->
                val left = it.first
                val right = it.second
                val isCorrect = isCorrectOrder(left, right)
                println("${i+1} Is correct: $isCorrect")
                sum += if (isCorrect == Trilean.TRUE) { i+1 } else { 0 }
            }
        println("Solution is $sum")
    }

    enum class Trilean {
        TRUE, FALSE, UNKNOWN
    }

    private fun isCorrectOrder(left: Node, right: Node): Trilean {
        if (left.isLeaf() && right.isLeaf()) {
            return if (left.value < right.value) { Trilean.TRUE } else if (left.value == right.value) { Trilean.UNKNOWN } else { Trilean.FALSE }
        } else if (!left.isLeaf() && !right.isLeaf()) {
            val leftChildren = left.children
            val rightChildren = right.children
            if (leftChildren.size > rightChildren.size) {
                for (i in 0..rightChildren.size-1) {
                    val isCorrect = isCorrectOrder(leftChildren[i], rightChildren[i])
                    if (isCorrect == Trilean.FALSE) {
                        return Trilean.FALSE
                    } else if (isCorrect == Trilean.TRUE) {
                        return Trilean.TRUE
                    }
                }
                return Trilean.FALSE
            } else if (leftChildren.size < rightChildren.size) {
                for (i in 0..leftChildren.size-1) {
                    val isCorrect = isCorrectOrder(leftChildren[i], rightChildren[i])
                    if (isCorrect == Trilean.FALSE) {
                        return Trilean.FALSE
                    } else if (isCorrect == Trilean.TRUE) {
                        return Trilean.TRUE
                    }
                }
                return Trilean.TRUE
            } else {
                for (i in 0..leftChildren.size-1) {
                    val isCorrect = isCorrectOrder(leftChildren[i], rightChildren[i])
                    if (isCorrect == Trilean.FALSE) {
                        return Trilean.FALSE
                    } else if (isCorrect == Trilean.TRUE) {
                        return Trilean.TRUE
                    }
                }
                return Trilean.UNKNOWN
            }
        } else if (left.isLeaf() && !right.isLeaf()) {
            val newNode = Node(children = mutableListOf(left))
            return isCorrectOrder(newNode, right)
        } else {
            val newNode = Node(children = mutableListOf(right))
            return isCorrectOrder(left, newNode)
        }
    }

    fun main() {
        val (lefts, rights) = readInFileCalledExamplePerLine()
        println("lefts: $lefts")
        println("rights: $rights")
        solve(lefts, rights)

        val twoNode = Node()
        val twoNodeInternal = Node(parent = twoNode)
        val two = Node(value = 2, parent = twoNodeInternal)
        twoNode.children.add(twoNodeInternal)
        twoNodeInternal.children.add(two)

        val sixNode = Node()
        val sixNodeInternal = Node(parent = sixNode)
        val six = Node(value = 6, parent = sixNodeInternal)
        sixNode.children.add(sixNodeInternal)
        sixNodeInternal.children.add(six)

        val all = lefts.plus(rights).plus(twoNode).plus(sixNode)
        val allSorted = all.sortedWith { a, b ->
            when (isCorrectOrder(a, b)) {
                Trilean.TRUE -> 1
                Trilean.FALSE -> -1
                Trilean.UNKNOWN -> 0
            }
        }.reversed()

        val indexTwo = allSorted.indexOf(twoNode) + 1
        val indexSix = allSorted.indexOf(sixNode) + 1
        println("Solution is ${indexTwo * indexSix}")
    }
}

fun main() {
    App().main()
}
