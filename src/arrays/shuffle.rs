use rand::Rng;

//@ To shuffle an array:
//@ 1. We iterate through the array provided once.
//@ 2. and choose a random number from our current index to the last index of the array,
//@ 3. And then we swap them. We do this N times.
pub fn shuffle<T>(a: &mut [T]) {
    let mut trng = rand::thread_rng();
    for i in 0..a.len() {
        let r = trng.gen_range(i..a.len());
        a.swap(i, r);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        f64::consts::E,
    };

    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use super::*;

    //@ Note that it's not possible to unit test a shuffling function
    //@ because there's no simple way to map input to output:
    //@ i.e. One shuffle should ideally be as likely as any other permutation
    //@ so we could run the unit test (n!) times, and check that the distribution rolls each
    //@ permutation once, but it's extremely unlikely that the unit test passes.
    //@ ```rust
    //@ #[test]
    //@ fn example() {
    //@     let mut input = &[5, 6, 7, 8, 9, 1, 2, 3, 4];
    //@     let mut cloned = *input;
    //@     sample(&mut cloned);
    //@     assert_ne!(&cloned, input); This won't always pass
    //@ }
    //@ ```

    //@ To test this function, we know the ideal outcome (the number of trials / permutations)
    //@ we can then give the permutations and some fixed error (0.05) to a function that will
    //@ calculate the chi-squared goodness of fit. We can then make sure that all the permutations
    //@ total difference is less than that number and if so, we have a 95% chance that the
    //@ distribution is randomly distributed.
    #[quickcheck]
    fn test_shuffle_uniformity(input: HashSet<i32>) -> bool {
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
            shuffle(&mut input);
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
