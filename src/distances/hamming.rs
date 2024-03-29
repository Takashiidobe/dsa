use num_traits::PrimInt;

pub fn hamming_distance<T: PrimInt>(a: T, b: T) -> u32 {
    (a ^ b).count_ones()
}
