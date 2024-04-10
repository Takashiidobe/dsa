pub struct FenwickTree {
    pub tree: Vec<i32>,
    pub size: usize,
}

impl FenwickTree {
    // Initializes a Fenwick Tree with a given size
    pub fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1],
            size,
        }
    }

    // Updates the value at a given index by adding 'value' to it
    pub fn update(&mut self, mut index: usize, value: i32) {
        while index <= self.size {
            self.tree[index] += value;
            index += index & index.wrapping_neg(); // Move to the next relevant node
        }
    }

    // Queries the cumulative sum up to a given index
    pub fn query(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & index.wrapping_neg(); // Move to the parent node
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

        assert_eq!(ft.query(3), 9);
        assert_eq!(ft.query(5), 9);
    }
}
