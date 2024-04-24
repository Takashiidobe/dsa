use num_traits::PrimInt;

pub fn hamming_distance<T: PrimInt>(a: T, b: T) -> u32 {
    (a ^ b).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    //@ Hamming distances should be symmetric
    #[quickcheck]
    fn prop_hamming_distance_is_symmetric(x: u32, y: u32) -> bool {
        hamming_distance(x, y) == hamming_distance(y, x)
    }

    //@ The hamming distance should be zero always for any number
    #[quickcheck]
    fn prop_hamming_distance_zero_iff_equal(x: u32) -> bool {
        (hamming_distance(x, x) == 0)
    }

    //@ distance should be between the bit length of the numbers.
    #[quickcheck]
    fn prop_hamming_distance_non_negative_and_bounded(x: u32, y: u32) -> bool {
        let distance = hamming_distance(x, y);
        (0..=32).contains(&distance)
    }
}
