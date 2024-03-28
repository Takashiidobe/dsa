use std::collections::{HashMap, HashSet, VecDeque};

/// An implementation of Kosaraju's algorithm, which finds the strongly connected components (SCCs)
/// of a graph, where any node in an SCC can reach any other node in the SCC.
///
/// Input: This function takes a dictionary where the key is the node and the value is a list
/// of its children nodes.
///
/// Output: A dictionary where the key is the root of each SCC, and the value is the list of
/// connected nodes in the SCCs.
///
/// The algorithm takes 4 steps.
///
/// 1. the algorithm allocates a few data structures required later.
///     - A dictionary of outdegrees `outdegrees`, where each node is the key and its values are the nodes it is connected to.
///     - A dictionary of indegrees `indegrees`, which is the opposite. Every node notes which nodes connect to it.
///     - A set `visited`, which is used to avoid cycles when visiting the graph.
///     - A deque `l`, which is used to keep track of the order in which nodes are visited.
///     - A set `assigned`, which is used to keep track of the nodes assigned to an SCC.
///     - A dictionary `islands` which is used to note the SCCs and is returned to the caller.
///
/// 2. the algorithm populates the outdegrees and indegrees. Since the input
/// is already an outdegree graph, we iterate through each node's edges and do the following:
///     For the outdegree graph, set the outdegree's key as the node and add the edge to its values.
///     For the indegree graph, set the indegree's key as the edge and add the node to its values.
///
/// 3. the algorithm creates a visit function, which is called on every node in the graph.
/// `Visit(node)` is a recursive function that does the following:
///     If `node` is not in the visited set:
///         Add `node` to `visited`.
///         For each outdegree of node, call `visit(outdegree)`.
///         Prepend node to `l`.
///     Else:
///         Do nothing
///
/// 4. the algorithm creates an assign function, which is called on every node in l in order.
/// `Assign(node, root)` is a recursive function that does the following:
///     If a `node` is not in the assigned set:
///         Add `node` to `assigned`.
///         Assign `node` to `root`'s SCC.
///         For each indegree of node, call `assign(indegree, root)`.
///     Else:
///         Do nothing
pub fn kosaraju(graph: HashMap<u32, Vec<u32>>) -> HashMap<u32, Vec<u32>> {
    let mut outdegrees: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut indegrees: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut visited = HashSet::new();
    let mut l = VecDeque::new();
    let mut assigned = HashSet::new();
    let mut islands: HashMap<u32, Vec<u32>> = HashMap::new();

    for (node, edges) in &graph {
        for edge in edges {
            outdegrees.entry(*node).or_default().push(*edge);
            indegrees.entry(*edge).or_default().push(*node);
        }
    }

    fn visit(
        node: u32,
        visited: &mut HashSet<u32>,
        outdegrees: &HashMap<u32, Vec<u32>>,
        l: &mut VecDeque<u32>,
    ) {
        if !visited.contains(&node) {
            visited.insert(node);
            let neighbors = outdegrees.get(&node).cloned().unwrap_or_default();
            for neighbor in neighbors {
                visit(neighbor, visited, outdegrees, l);
            }
            l.push_front(node);
        }
    }

    for node in graph.keys() {
        visit(*node, &mut visited, &outdegrees, &mut l);
    }

    fn assign(
        node: u32,
        root: u32,
        assigned: &mut HashSet<u32>,
        indegrees: &HashMap<u32, Vec<u32>>,
        islands: &mut HashMap<u32, Vec<u32>>,
    ) {
        if !assigned.contains(&node) {
            assigned.insert(node);
            islands.entry(root).or_default().push(node);
            let neighbors = indegrees.get(&node).cloned().unwrap_or_default();
            for neighbor in neighbors {
                assign(neighbor, root, assigned, indegrees, islands);
            }
        }
    }

    for node in l {
        assign(node, node, &mut assigned, &indegrees, &mut islands);
    }

    islands
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use quickcheck_macros::quickcheck;

    #[test_fuzz::test_fuzz]
    fn _kosaraju(graph: HashMap<u32, Vec<u32>>) -> HashMap<u32, Vec<u32>> {
        kosaraju(graph)
    }

    fn sort_for_test(input: HashMap<u32, Vec<u32>>) -> Vec<Vec<u32>> {
        let mut sorted: Vec<Vec<_>> = input.values().cloned().collect();
        sorted.iter_mut().for_each(|i| i.sort());
        sorted.sort();
        sorted
    }

    #[test]
    fn empty() {
        let input = HashMap::from([(0, vec![])]);

        assert_yaml_snapshot!(sort_for_test(kosaraju(input)));
    }

    #[test]
    fn example() {
        let input = HashMap::from([
            (0, Vec::from([1, 2])),
            (1, Vec::from([0, 2])),
            (2, Vec::from([0, 1])),
            (3, Vec::from([4])),
            (4, Vec::from([5])),
            (5, Vec::from([3])),
            (6, Vec::from([7])),
            (7, Vec::default()),
        ]);

        assert_yaml_snapshot!(sort_for_test(kosaraju(input)));
    }

    #[quickcheck]
    fn verify_all_islands(input: HashMap<u32, Vec<u32>>) -> bool {
        let cloned_input = input.clone();
        let mut unique_vals: HashSet<&u32> = HashSet::from_iter(cloned_input.values().flatten());
        unique_vals.extend(cloned_input.keys());
        let result = kosaraju(input);

        // every unique val should be in the results
        let all_islands: HashSet<&u32> = HashSet::from_iter(result.values().flatten());
        unique_vals == all_islands
    }
}
