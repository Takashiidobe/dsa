pub struct SegmentTree {
    buf: Vec<u32>,
    len: usize,
}

impl SegmentTree {
    pub fn new(arr: &[u32]) -> Self {
        let len = arr.len();
        let mut buf = vec![u32::default(); 2 * len];
        buf[len..2 * len].copy_from_slice(arr);
        for i in (1..len).rev() {
            buf[i] = buf[2 * i].min(buf[2 * i + 1]);
        }

        SegmentTree { buf, len }
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> u32 {
        l += self.len - 1;
        r += self.len - 1;
        let mut ans = self.buf[l];
        while l <= r {
            if l % 2 == 1 {
                ans = ans.min(self.buf[l]);
                l += 1;
            }
            if r % 2 == 0 {
                ans = ans.min(self.buf[r]);
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        ans
    }

    pub fn update(&mut self, mut idx: usize, val: u32) {
        idx += self.len - 1;
        self.buf[idx] = val;
        idx /= 2;

        while idx != 0 {
            self.buf[idx] = self.buf[2 * idx].min(self.buf[2 * idx + 1]);
            idx /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[test]
    fn ex1() {
        let mut tree = SegmentTree::new(&[4, 3, 2, 8, 5, 1, 2, 1]);

        assert_eq!(tree.query(1, 7), 1);
    }

    //@ To test that our implementation is correct, here's a property test.
    //@ We want to generate some input and then verify manually that minimums are being properly
    //@ calculated by the segment tree.
    #[quickcheck]
    fn segment_min(input: Vec<u32>) -> bool {
        if input.len() <= 1 {
            return true;
        }
        let len = input.len();

        //@ To do that, we generate a random number of numbers, and then insert them all into the
        //@ segment tree.
        let mut tree = SegmentTree::new(&input);

        //@ afterwards, we query the tree for each index in the array, making sure it correctly
        //@ calculates the minimum by matching it to a naive O(n^2) implementation.
        //@ If they match, then we've implemented it properly.
        for i in 0..len {
            for j in (i + 1)..len {
                if *input[i..j].iter().min().unwrap() != tree.query(i, j) {
                    return false;
                }
            }
        }
        true
    }
}
