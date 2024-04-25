use std::{slice::Iter, vec::IntoIter};

use rand::{thread_rng, Rng};

//@ Online sampling from a stream
pub fn sample<T>(mut iterator: impl IntoIterator<Item = T>, k: usize) -> Vec<T> {
    let mut trng = thread_rng();
    let mut item_count = 0;
    let mut reservoir = vec![];
    for item in iterator {
        item_count += 1;

        if item_count <= k {
            reservoir.push(item);
        } else {
            let replace_index = trng.gen_range(0..item_count);
            if replace_index < k {
                reservoir[replace_index] = item;
            }
        }
    }

    reservoir
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::sample;

    #[test]
    fn test() {
        //@ it takes some sample
        let mut nums = vec![1, 2, 3, 4, 5].into_iter();
        let res = sample(nums, 3);
        assert_ne!(vec![1, 2, 3], res);
    }

    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use super::*;

    //@ Note that it's not possible to unit test a sample function
    //@ because there's no simple way to map input to output:
    //@ i.e. One sample should ideally be as likely as any other permutation
    //@ so we could run the unit test (n!) times, and check that the distribution rolls each
    //@ permutation once, but it's extremely unlikely that the unit test passes.
    //@ ```rust
    //@ #[test]
    //@ fn example() {
    //@     let mut input = &[1,2,3,4];
    //@     let mut cloned = *input;
    //@     sample(&mut cloned, 3);
    //@     assert_ne!(&cloned, input); This won't always pass
    //@ }
    //@ ```

    //@ To test this function, we know the ideal outcome (the number of trials / permutations)
    //@ we can then give the permutations and some fixed error (0.05) to a function that will
    //@ calculate the chi-squared goodness of fit. We can then make sure that all the permutations
    //@ total difference is less than that number and if so, we have a 95% chance that the
    //@ distribution is randomly distributed.
    #[quickcheck]
    fn test_sample_uniformity(input: HashSet<i32>) -> bool {
        let mut input: Vec<_> = input.into_iter().collect();
        if input.len() != 4 {
            return true;
        }
        let mut permutations = HashMap::new();

        for perm in input.iter().cloned().permutations(input.len()) {
            permutations.insert(perm, 0);
        }

        let num_samples = 10000;
        for _ in 0..num_samples {
            sample(&mut input, 3);
            *permutations.entry(input.clone()).or_insert(0) += 1;
        }

        let expected = num_samples as f64 / permutations.len() as f64;
        let chi_squared: f64 = permutations
            .values()
            .map(|&count| {
                let diff = count as f64 - expected;
                diff * diff / expected
            })
            .sum();

        chi_squared < 35.17
    }
}
