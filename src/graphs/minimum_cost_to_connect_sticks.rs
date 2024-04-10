use std::{cmp::Reverse, collections::BinaryHeap};

pub fn minimum_cost(sticks: &[u32]) -> u32 {
    let mut heap = BinaryHeap::new();

    for stick in sticks {
        heap.push(Reverse(*stick));
    }

    let mut total_cost: u32 = 0;

    while heap.len() > 1 {
        let top = heap.pop().unwrap();
        let next_top = heap.pop().unwrap();
        let cost = top.0.saturating_add(next_top.0);
        total_cost = total_cost.saturating_add(cost);
        heap.push(Reverse(cost));
    }

    total_cost
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[test]
    fn ex1() {
        let cost = minimum_cost(&[2, 4, 3]);
        assert_eq!(cost, 14);
    }

    #[test]
    fn ex2() {
        let cost = minimum_cost(&[1, 8, 3, 5]);
        assert_eq!(cost, 30);
    }

    #[test]
    fn ex3() {
        let cost = minimum_cost(&[5]);
        assert_eq!(cost, 0);
    }

    #[quickcheck]
    fn verify(input: Vec<u32>) -> bool {
        let result = minimum_cost(&input);
        let max_item = input.iter().max().unwrap_or(&0);
        if input.len() > 1 {
            result >= *max_item
        } else {
            result == 0
        }
    }
}
