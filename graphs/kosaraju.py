graph = {
        0: [1,2],
        1: [0,2],
        2: [0,1],
        3: [4],
        4: [5],
        5: [3],
        6: [7],
        7: [],
    }

from collections import defaultdict, deque

def kosaraju(graph):
    """
    An implementation of Kosaraju's algorithm, which finds the strongly connected components (SCCs)
    of a graph, where any node in an SCC can reach any other node in the SCC. 

    Input: This function takes a dictionary where the key is the node and the value is a list
    of its children nodes.

    Output: A dictionary where the key is the root of each SCC, and the value is the list of 
    connected nodes in the SCCs.

    The algorithm takes 4 steps.

    1. the algorithm allocates a few data structures required later.

    A dictionary of outdegrees `outdegrees`, where each node is the key and its values are the nodes it is connected to.
    A dictionary of indegrees `indegrees`, which is the opposite. Every node notes which nodes connect to it.
    A set `visited`, which is used to avoid cycles when visiting the graph.
    A deque `l`, which is used to keep track of the order in which nodes are visited.
    A set `assigned`, which is used to keep track of the nodes assigned to an SCC.
    A dictionary `islands` which is used to note the SCCs and is returned to the caller.

    2. the algorithm populates the outdegrees and indegrees. Since the input
    is already an outdegree graph, we iterate through each node's edges and do the following:
        For the outdegree graph, set the outdegree's key as the node and add the edge to its values.
        For the indegree graph, set the indegree's key as the edge and add the node to its values.

    3. the algorithm creates a visit function, which is called on every node in the graph.
    `Visit(node)` is a recursive function that does the following:
        If `node` is not in the visited set:
            Add `node` to visited.
            For each outdegree of node, call `visit(outdegree)`.
            Prepend node to `l`.
        Else:
            Do nothing

    4. the algorithm creates an assign function, which is called on every node in l in order.
    `Assign(node, root)` is a recursive function that does the following:
        If a `node` is not in the assigned set:
            Assign `node` as belonging to `root`'s component.
            For each indegree of node, call `assign(indegree, root)`.
        Else:
            Do nothing
    """
    outdegrees = defaultdict(set)
    indegrees = defaultdict(set)
    visited = set()
    l = deque()
    assigned = set()
    islands = defaultdict(set)

    # create indegrees and outdegrees
    for node, edges in graph.items():
        for edge in edges:
            outdegrees[node].add(edge)
            indegrees[edge].add(node)

    def visit(node):
        if node not in visited:
            visited.add(node)
            for neighbor in outdegrees[node]:
                visit(neighbor)
            l.appendleft(node)

    for node in graph.keys():
        visit(node)

    def assign(node, root):
        if node not in assigned:
            assigned.add(node)
            islands[root].add(node)
            for neighbor in indegrees[node]:
                assign(neighbor, root)

    for node in l:
        assign(node, node)

    return islands

print(kosaraju(graph))
