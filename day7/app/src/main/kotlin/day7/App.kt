package day7

class App {
    data class Node(val name: String, val weight: Int, val children: MutableList<Node>, val parent: Node? = null) {
        override fun toString(): String {
            return "Node(name='$name', weight=$weight, children=${children.map { it.name }}, parent=${parent?.name})"
        }
    }

    val TOTAL_SIZE = 70_000_000
    val NEEDED_SIZE = 30_000_000

    fun readInFileCalledInputPerLine(): List<String> {
        val input = this::class.java.getResource("/input").readText()
        val inputLines = input.split("\r?\n|\n".toRegex())
        println(inputLines.size)
        return inputLines
    }

    fun traverseInputAndPopulateNodes(input: List<String>, node: Node) {
        var workingNode = node
        input.forEach { line ->
            if (line.startsWith("$ cd ..")) {
                workingNode = workingNode.parent!!
            } else if (line.startsWith("$ ls")) {
                
            } else if (line.startsWith("$ cd ")) {
                val dirName = line.split(" ")[2]
                workingNode = workingNode.children.find { it.name == dirName }!!
            } else if (line.startsWith("dir ")) {
                val dirName = line.split(" ")[1]
                val newDirNode = Node(dirName, 0, mutableListOf(), workingNode)
                workingNode.children.add(newDirNode)
            } else {
                val fileName = line.split(" ")[1]
                val fileSize = line.split(" ")[0].toInt()
                val newFileNode = Node(fileName, fileSize, mutableListOf(), workingNode)
                workingNode.children.add(newFileNode)
            }
        }
    }

    fun sizeOfNode(node: Node): Int {
        return node.children.map { sizeOfNode(it) }.sum() + node.weight
    }

    fun findNodesWithWeightAtMostHundredThousand(node: Node): List<Node> {
        val nodes = mutableListOf<Node>()
        if (sizeOfNode(node) <= 100000) {
            nodes.add(node)
        }
        node.children.forEach { nodes.addAll(findNodesWithWeightAtMostHundredThousand(it)) }
        return nodes.filter { it.weight == 0 }
    }

    fun findNodesWithWeightAboveThreshold(node: Node, threshold: Int): List<Node> {
        val nodes = mutableListOf<Node>()
        if (sizeOfNode(node) >= threshold) {
            nodes.add(node)
        }
        node.children.forEach { nodes.addAll(findNodesWithWeightAboveThreshold(it, threshold)) }
        return nodes.filter { it.weight == 0 }
    }

    fun findSmallestDirectoryToDeleteToGetNeededSize(node: Node): String {
        val currentlyUsed = sizeOfNode(node)
        println("Currently used: $currentlyUsed")
        val currentlyFree = TOTAL_SIZE - currentlyUsed 
        println("Currently free: $currentlyFree")
        val needToFree = NEEDED_SIZE - currentlyFree
        println("Need to free: $needToFree")
        val nodes = findNodesWithWeightAboveThreshold(node, needToFree)
        val nodeToDelete = nodes.minByOrNull { sizeOfNode(it) }!!
        println("Node to delete: ${nodeToDelete.name} with size ${sizeOfNode(nodeToDelete)}")
        println("Free space after deleting: ${currentlyFree + sizeOfNode(nodeToDelete)}")
        println(nodeToDelete)
        return nodeToDelete.name
    }

    fun solve() {
        println("Starting to solve first..")
        val node = Node("/", 0, mutableListOf())
        var input = readInFileCalledInputPerLine()
        input = input.subList(1, input.size)
        traverseInputAndPopulateNodes(input, node)
        println(findNodesWithWeightAtMostHundredThousand(node).map { sizeOfNode(it) }.sum())
    }

    fun solve2() {
        println("Starting to solve second..")
        val node = Node("/", 0, mutableListOf())
        var input = readInFileCalledInputPerLine()
        input = input.subList(1, input.size)
        traverseInputAndPopulateNodes(input, node)
        println(findSmallestDirectoryToDeleteToGetNeededSize(node))
    }
}

fun main() {
    App().solve()
    App().solve2()
}
