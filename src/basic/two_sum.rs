use std::collections::HashMap;
use std::hash::Hash;

use num_traits::PrimInt;

pub fn two_sum<N: PrimInt + Hash>(nums: &[N], target: N) -> Option<(usize, usize)> {
    let mut lookup: HashMap<N, usize> = HashMap::default();

    for (i, num) in nums.iter().enumerate() {
        let offset = target.saturating_sub(*num);
        match lookup.get(&offset) {
            Some(prev_i) => match offset.checked_add(num) {
                Some(res) if res == target => return Some((*prev_i, i)),
                _ => {
                    lookup.insert(*num, i);
                }
            },
            None => {
                lookup.insert(*num, i);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn verify(input: Vec<i32>, target: i32) -> bool {
        if input.len() > 100 {
            return true;
        }

        match two_sum(&input, target) {
            Some((left_i, right_i)) => input[left_i].saturating_add(input[right_i]) == target,
            None => true,
        }
    }
}
