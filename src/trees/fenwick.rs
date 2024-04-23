use num_traits::PrimInt;

//@ A [Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree) is a data structure for
//@ efficiently calculating prefix sums.
//@ It can calculate prefix sums, update them, etc in O(log(N)) time like a normal tree.
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
            self.tree[index] = self.tree[index].saturating_add(value);
            //@ `index & index.wrapping_neg()` gets the lowest set bit of index.
            //@ Take an index of `12`, which would be `1100`. By taking `index &
            //@ index.wrapping_neg()`, and adding it to index, we get 16, or `10000`
            //@ Thus, this always increments in powers of 2, so, 1, 2, 4, 8, 16, 32...
            index += index & index.wrapping_neg();
        }
    }

    //@ Queries the cumulative sum up to a given index
    pub fn query(&self, mut index: usize) -> N {
        let mut sum: N = N::zero();
        while index > 0 {
            sum = sum.saturating_add(self.tree[index]);
            //@ `index & index.wrapping_neg()` gets the lowest set bit of index.
            //@ Then we subtract it from index to get to the nearest smaller power of 2.
            //@ Thus, this decrements in powers of 2, like 32, 16, 8, 4, 2, 1, 0.
            index -= index & index.wrapping_neg();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[test]
    fn ex1() {
        let mut ft = FenwickTree::new(10);

        for i in 1..=5 {
            ft.update(i, i);
        }

        assert_eq!(ft.query(1), 1);
        assert_eq!(ft.query(3), 6);
        assert_eq!(ft.query(5), 15);
    }

    //@ To test that our implementation is correct, here's a property test.
    //@ We want to generate some input and then verify manually that prefix sums are being properly
    //@ calculated by the fenwick tree.
    #[quickcheck]
    fn sum_in_bounds(input: Vec<usize>) -> bool {
        let len = input.len();

        //@ To do that, we generate a random number of variables, and then insert them all into the
        //@ fenwick tree.
        let mut ft = FenwickTree::new(len + 1);
        for (i, val) in input.iter().enumerate() {
            ft.update(i + 1, *val);
        }

        //@ afterwards, we query the tree for each index in the array, making sure it correctly
        //@ calculates the prefix sum by matching it to a naive O(n^2) implementation.
        //@ If they match, then we've implemented it properly.
        (0..len)
            .map(|i| {
                let mut total: usize = 0;
                for j in (0..i).take(i) {
                    total = total.saturating_add(input[j]);
                }
                total == ft.query(i)
            })
            .all(|t| t)
    }
}
