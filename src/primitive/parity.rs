use lazy_static::lazy_static;

//@ count the number of even/odd bits
//@ Takes O(n) time, where n is the width of the word
//@ Also somewhat slow:
//@ `test primitive::parity::tests::bench_parity1 ... bench:     300,663 ns/iter (+/- 8,471)`
pub fn parity1(mut x: u64) -> u64 {
    let mut result = 0;
    while x > 0 {
        result ^= x & 1;
        x >>= 1
    }
    result
}

//@ Takes O(k) time, where k is the number of bits set to 1 (can be n).
//@ The book says this is faster, and on my computer it is.
//@ `test primitive::parity::tests::bench_parity2 ... bench:     177,997 ns/iter (+/- 3,596)`
pub fn parity2(mut x: u64) -> u64 {
    let mut result = 0;
    while x > 0 {
        result ^= x & 1;
        x &= x - 1
    }
    result
}

const PRECOMPUTED_PARITY_SIZE: usize = 1 + (u16::MAX as usize);
const BIT_MASK: u64 = 0xFFFF;

//@ The precomputed parity table itself.
lazy_static! {
    static ref PRECOMPUTED_PARITY: [u64; PRECOMPUTED_PARITY_SIZE] = {
        let mut precomputed_parity = [0; PRECOMPUTED_PARITY_SIZE];

        for i in (0..).take(PRECOMPUTED_PARITY_SIZE) {
            precomputed_parity[i] = i.count_ones() as u64;
        }

        precomputed_parity
    };
}

//@ This method uses a lookup table and is a bit faster.
//@ This also seems to be a bit faster.
//@ `test primitive::parity::tests::bench_parity3 ... bench:     120,589 ns/iter (+/- 1,362)`
pub fn parity3(x: u64) -> u64 {
    PRECOMPUTED_PARITY[(x & BIT_MASK) as usize]
        ^ PRECOMPUTED_PARITY[(x >> 16 & BIT_MASK) as usize]
        ^ PRECOMPUTED_PARITY[(x >> 32 & BIT_MASK) as usize]
        ^ PRECOMPUTED_PARITY[(x >> 48 & BIT_MASK) as usize]
}

//@ This one is O(log(n)) time, since it divides and conquers the input.
//@ `test primitive::parity::tests::bench_parity4 ... bench:      61,933 ns/iter (+/- 514)`
pub fn parity4(mut x: u64) -> u64 {
    x ^= x >> 32;
    x ^= x >> 16;
    x ^= x >> 8;
    x ^= x >> 4;
    x ^= x >> 2;
    x ^= x >> 1;

    x & 0x1
}

//@ Using the intrinsics builtin. This is the fastest by far.
//@ `test primitive::parity::tests::bench_parity5 ... bench:      53,765 ns/iter (+/- 10,017)`
pub fn parity5(x: u64) -> u64 {
    x.count_ones() as u64
}

#[cfg(test)]
mod tests {
    use crate::PEAK_ALLOC;

    use super::{parity1, parity2, parity3, parity4, parity5};
    use test::{black_box, Bencher};

    #[test]
    fn memory_usage_parity_1() {
        for i in 0..=u16::MAX {
            parity1(i as u64);
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

    #[test]
    fn memory_usage_parity_5() {
        for i in 0..=u16::MAX {
            parity5(i as u64);
        }
        println!("{}", PEAK_ALLOC.peak_usage());
    }

    #[bench]
    fn bench_parity1(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..=u16::MAX {
                black_box(parity1(i as u64));
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

    #[bench]
    fn bench_parity5(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..=u16::MAX {
                black_box(parity5(i as u64));
            }
        })
    }
}
