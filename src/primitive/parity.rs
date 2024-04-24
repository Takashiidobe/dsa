use std::{
    convert::From,
    ops::{AddAssign, BitAnd, BitAndAssign, BitXorAssign, Shr, ShrAssign},
};

use lazy_static::lazy_static;
use num_traits::PrimInt;

//@ count the number of even/odd bits
//@ Takes O(n) time, where n is the width of the word
//@ Also somewhat slow:
//@ `test primitive::parity::tests::bench_parity1 ... bench:     300,663 ns/iter (+/- 8,471)`
pub fn count_bits<N: PrimInt + ShrAssign + AddAssign>(mut x: N) -> N {
    let mut result = N::zero();
    while x > N::zero() {
        result += x & N::one();
        x >>= N::one();
    }
    result
}

//@ Takes O(k) time, where k is the number of bits set to 1 (can be n).
//@ The book says this is faster, and on my computer it is.
//@ `test primitive::parity::tests::bench_parity2 ... bench:     177,997 ns/iter (+/- 3,596)`
pub fn parity2<N: PrimInt + ShrAssign + BitXorAssign + BitAndAssign>(mut x: N) -> N {
    let mut result = N::zero();
    while x > N::zero() {
        result ^= N::one();
        x &= (x - N::one());
    }
    result
}

//@ This one is O(log(n)) time, since it divides and conquers the input.
//@ `test primitive::parity::tests::bench_parity4 ... bench:      61,933 ns/iter (+/- 514)`
pub fn parity3<N: PrimInt + BitXorAssign + BitAnd>(mut x: N) -> N {
    x ^= x >> 32;
    x ^= x >> 16;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;

    x & N::one()
}

//@ Using the intrinsics builtin. This is the fastest by far.
//@ `test primitive::parity::tests::bench_parity5 ... bench:      53,765 ns/iter (+/- 10,017)`
pub fn parity4<N: PrimInt + From<u32>>(mut x: N) -> N {
    if x.count_ones() % 2 != 0 {
        1.into()
    } else {
        0.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::PEAK_ALLOC;

    use super::{count_bits, parity2, parity3, parity4};
    use quickcheck_macros::quickcheck;
    use test::{black_box, Bencher};

    #[quickcheck]
    fn count_bits_is_correct(num: u64) -> bool {
        count_bits(num) as u32 == num.count_ones()
    }

    #[quickcheck]
    fn parity2_is_correct(num: u64) -> bool {
        parity2(num) == parity4(num)
    }

    #[quickcheck]
    fn parity3_is_correct(num: u64) -> bool {
        parity3(num) == parity4(num)
    }

    #[test]
    fn memory_usage_count_bits() {
        for i in 0..=u16::MAX {
            count_bits(i as u64);
        }
        println!("{}", PEAK_ALLOC.peak_usage());
    }

    #[test]
    fn memory_usage_parity_2() {
        for i in 0..=u16::MAX {
            parity2(i as u64);
        }
        println!("{}", PEAK_ALLOC.peak_usage());
    }

    #[test]
    fn memory_usage_parity_3() {
        for i in 0..=u16::MAX {
            parity3(i as u64);
        }
        println!("{}", PEAK_ALLOC.peak_usage());
    }

    #[test]
    fn memory_usage_parity_4() {
        for i in 0..=u16::MAX {
            parity4(i as u64);
        }
        println!("{}", PEAK_ALLOC.peak_usage());
    }

    #[bench]
    fn bench_parity1(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..=u16::MAX {
                black_box(count_bits(i as u64));
            }
        })
    }

    #[bench]
    fn bench_parity2(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..=u16::MAX {
                black_box(parity2(i as u64));
            }
        })
    }

    #[bench]
    fn bench_parity3(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..=u16::MAX {
                black_box(parity3(i as u64));
            }
        })
    }

    #[bench]
    fn bench_parity4(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..=u16::MAX {
                black_box(parity4(i as u64));
            }
        })
    }
}
