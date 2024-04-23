use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    height: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: &Option<Box<Node<T>>>) -> i32 {
        node.as_ref().map_or(0, |node| node.height)
    }

    fn update_height(&mut self) {
        self.height = i32::max(Self::height(&self.left), Self::height(&self.right)) + 1;
    }

    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) - Self::height(&self.right)
    }

    fn rotate_right(mut self) -> Box<Node<T>> {
        let mut new_root = self.left.take().unwrap();
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(Box::new(self));
        new_root.update_height();
        new_root
    }

    fn rotate_left(mut self) -> Box<Node<T>> {
        let mut new_root = self.right.take().unwrap();
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(Box::new(self));
        new_root.update_height();
        new_root
    }

    fn balance(mut self) -> Box<Node<T>> {
        self.update_height();
        let balance = self.balance_factor();
        if balance > 1 {
            if self.left.as_ref().unwrap().balance_factor() < 0 {
                self.left = Some(self.left.take().unwrap().rotate_left());
            }
            self.rotate_right()
        } else if balance < -1 {
            if self.right.as_ref().unwrap().balance_factor() > 0 {
                self.right = Some(self.right.take().unwrap().rotate_right());
            }
            self.rotate_left()
        } else {
            Box::new(self)
        }
    }

    fn insert(self, value: T) -> Box<Node<T>> {
        let mut node = Box::new(self);
        match value.cmp(&node.value) {
            Ordering::Less => match node.left {
                Some(left) => node.left = Some(left.insert(value)),
                None => node.left = Some(Box::new(Node::new(value))),
            },
            Ordering::Equal | Ordering::Greater => match node.right {
                Some(right) => node.right = Some(right.insert(value)),
                None => node.right = Some(Box::new(Node::new(value))),
            },
        }
        node.balance()
    }

    fn contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => match &self.left {
                Some(left) => left.contains(value),
                None => false,
            },
            Ordering::Equal => true,
            Ordering::Greater => match &self.right {
                Some(right) => right.contains(value),
                None => false,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct AVLTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> AVLTree<T> {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self, value: T) {
        match self.root.take() {
            Some(root) => self.root = Some(root.insert(value)),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    fn contains(&self, value: &T) -> bool {
        match &self.root {
            Some(root) => root.contains(value),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for AVLTree<i32> {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut tree = AVLTree::new();
            let size = usize::arbitrary(g) % 1000; // Limit size to 1000 for performance
            for _ in 0..size {
                tree.insert(i32::arbitrary(g));
            }
            tree
        }
    }

    #[quickcheck]
    fn avl_tree_contains_all_inserted_elements(tree: AVLTree<i32>) -> bool {
        let mut elements = vec![];
        // Collect all elements from the tree to a vector (implement this function based on your AVL tree structure)
        collect_elements(&tree.root, &mut elements);

        // Check all elements are found in the tree
        elements.iter().all(|&elem| tree.contains(&elem))
    }

    #[quickcheck]
    fn avl_tree_is_always_balanced(tree: AVLTree<i32>) -> bool {
        is_balanced(&tree.root)
    }

    // Helper function to collect elements from the tree
    fn collect_elements(node: &Option<Box<Node<i32>>>, elements: &mut Vec<i32>) {
        if let Some(ref n) = node {
            elements.push(n.value);
            collect_elements(&n.left, elements);
            collect_elements(&n.right, elements);
        }
    }

    // Helper function to check if the tree is balanced
    fn is_balanced(node: &Option<Box<Node<i32>>>) -> bool {
        node.as_ref().map_or(true, |n| {
            let left_height = Node::height(&n.left);
            let right_height = Node::height(&n.right);
            let balance_factor = (left_height - right_height).abs();
            balance_factor <= 1 && is_balanced(&n.left) && is_balanced(&n.right)
        })
    }
}
