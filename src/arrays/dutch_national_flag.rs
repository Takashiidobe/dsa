use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Color {
    Red,
    White,
    Blue,
}

pub fn dutch_flag_partition(flag: &[Color]) -> Vec<Color> {
    let (mut red_count, mut white_count, mut blue_count) = (0, 0, 0);
    for color in flag.iter() {
        match color {
            Color::Red => red_count += 1,
            Color::White => white_count += 1,
            Color::Blue => blue_count += 1,
        }
    }

    let mut sorted_flag = Vec::with_capacity(flag.len());

    for i in 0..flag.len() {
        if i < red_count {
            sorted_flag.push(Color::Red);
        } else if i < red_count + white_count {
            sorted_flag.push(Color::White);
        } else {
            sorted_flag.push(Color::Blue);
        }
    }
    sorted_flag
}

pub fn dutch_flag_partition_inplace(flag: &mut [Color]) {
    let (mut red_count, mut white_count, mut blue_count) = (0, 0, 0);
    for color in flag.iter() {
        match color {
            Color::Red => red_count += 1,
            Color::White => white_count += 1,
            Color::Blue => blue_count += 1,
        }
    }

    for i in 0..flag.len() {
        if i < red_count {
            flag[i] = Color::Red;
        } else if i < red_count + white_count {
            flag[i] = Color::White;
        } else {
            flag[i] = Color::Blue;
        }
    }
}

pub fn dutch_flag_sort_inplace(flag: &mut [Color]) {
    let (mut low, mut mid, mut high) = (0, 0, flag.len() - 1);

    while mid <= high {
        if flag[high] == Color::Red {
            flag.swap(low, mid);
            low += 1;
            mid += 1;
        } else if flag[mid] == Color::White {
            mid += 1;
        } else {
            flag.swap(mid, high);
            high -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use quickcheck_macros::quickcheck;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
    use test::Bencher;

    use super::*;

    impl Distribution<Color> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
            match rng.gen_range(0..=2) {
                0 => Color::Red,
                1 => Color::White,
                _ => Color::Blue,
            }
        }
    }

    impl Arbitrary for Color {
        fn arbitrary(g: &mut Gen) -> Self {
            match g.choose(&[Color::Red, Color::White, Color::Blue]) {
                Some(&color) => color,
                _ => unreachable!(),
            }
        }
    }

    lazy_static! {
        static ref FLAGS: Vec<Vec<Color>> = {
            let mut rng = rand::thread_rng();
            let mut res = vec![];

            for i in 0..5 {
                res.push((0..1000).map(|_| rng.gen()).collect());
            }
            res
        };
    }

    #[quickcheck]
    fn propcheck_flag(input: Vec<Color>) -> bool {
        let mut sorted_clone = input.clone();
        sorted_clone.sort();
        dutch_flag_partition(&input) == sorted_clone
    }

    #[bench]
    fn flag(b: &mut Bencher) {
        let flags = FLAGS.clone();
        b.iter(|| {
            for flag in flags.iter() {
                dutch_flag_partition(flag);
            }
        })
    }

    #[bench]
    fn flag_sort_inplace(b: &mut Bencher) {
        let mut flags = FLAGS.clone();
        b.iter(|| {
            for flag in flags.iter_mut() {
                dutch_flag_sort_inplace(flag);
            }
        })
    }

    #[bench]
    fn flag_inplace(b: &mut Bencher) {
        let mut flags = FLAGS.clone();
        b.iter(|| {
            for flag in flags.iter_mut() {
                dutch_flag_partition_inplace(flag);
            }
        })
    }

    #[bench]
    fn builtin_flag_sort(b: &mut Bencher) {
        let mut flags = FLAGS.clone();
        b.iter(|| {
            for flag in flags.iter_mut() {
                flag.sort_unstable();
            }
        })
    }
}
