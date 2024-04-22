use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn minimum_cost(n: u32, connections: Vec<(u32, u32, u32)>) -> Option<u32> {
    let mut graph: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();

    for (u, v, cost) in connections {
        graph.entry(u).or_default().push((cost, v));
        graph.entry(v).or_default().push((cost, u));
    }

    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((0, 1)));
    let mut visited = HashSet::new();
    let mut total_cost: u32 = 0;

    while !min_heap.is_empty() && visited.len() < n as usize {
        if let Some(Reverse((cost, u))) = min_heap.pop() {
            if visited.contains(&u) {
                continue;
            }
            visited.insert(u);
            total_cost = total_cost.saturating_add(cost);

            for (edge_cost, v) in graph.entry(u).or_default() {
                if !visited.contains(v) {
                    min_heap.push(Reverse((*edge_cost, *v)));
                }
            }
        }
    }

    if visited.len() == n as usize {
        Some(total_cost)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;
    use quickcheck_macros::quickcheck;

    #[test]
    fn ex1() {
        let res = minimum_cost(3, vec![(1, 2, 5), (1, 3, 6), (2, 3, 1)]);

        assert_eq!(res, Some(6));
    }

    #[test]
    fn ex2() {
        let res = minimum_cost(4, vec![(1, 2, 3), (3, 4, 4)]);

        assert_eq!(res, None);
    }

    #[test]
    fn ex3() {
        let res = minimum_cost(4, vec![(1, 2, 3), (3, 4, 4)]);

        assert_eq!(res, None);
    }

    #[quickcheck]
    fn verify(n: u32, input: Vec<(u32, u32, u32)>) -> bool {
        minimum_cost(n, input);
        // I can't think of a property that should hold, so just fuzzing.
        true
    }
}
