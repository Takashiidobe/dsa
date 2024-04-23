use num_traits::PrimInt;

//@ A [Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree) is a data structure for
//@ efficiently calculating prefix sums.
//@ It can calculate prefix sums, update them, etc in log(N) time like a normal tree.
pub struct FenwickTree<PrimInt> {
    pub tree: Vec<PrimInt>,
    pub size: usize,
}

impl<N: PrimInt> FenwickTree<N> {
    //@ Initializes a Fenwick Tree with a given size
    //@ Note that a fenwick tree is "one-indexed", which simplifies the implementation.
    pub fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![N::zero(); size + 1],
            size,
        }
    }

    //@ Updates the value at a given index by adding 'value' to it
    pub fn update(&mut self, mut index: usize, value: N) {
        while index <= self.size {
            self.tree[index] = self.tree[index] + value;
            //@ index & index.wrapping_neg() gets the lowest set bit of index.
            //@ Take an index of `12`, which would be `1100`. By taking index &
            //@ index.wrapping_neg(), and adding it to index, we get 16, or `10000`
            //@ Thus, this always increments in powers of 2, so, 1, 2, 4, 8, 16, 32...
            index += index & index.wrapping_neg();
        }
    }

    //@ Queries the cumulative sum up to a given index
    pub fn query(&self, mut index: usize) -> N {
        let mut sum: N = N::zero();
        while index > 0 {
            sum = sum + self.tree[index];
            //@ index & index.wrapping_neg() gets the lowest set bit of index.
            //@ Then we subtract it from index to get to the nearest smaller power of 2.
            //@ Thus, this decrements in powers of 2, like 32, 16, 8, 4, 2, 1, 0.
            index -= index & index.wrapping_neg();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut ft = FenwickTree::new(10);

        ft.update(1, 4);
        ft.update(3, 5);

        assert_eq!(ft.query(1), 4);
        assert_eq!(ft.query(3), 9);
        assert_eq!(ft.query(5), 9);
    }
}
