use super::tree_node::BSTNode;

pub fn in_order(root: &BSTNode) -> Vec<i32> {
    match root {
        Some(node) => {
            let borrowed_node = node.borrow();
            let val = borrowed_node.val;
            let mut left = in_order(&borrowed_node.left);
            left.push(val);
            let right = in_order(&borrowed_node.right);
            left.extend(right);
            left
        }
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::{btree, trees::in_order::in_order, trees::tree_node::TreeNode};

    #[test]
    fn ex() {
        let tree = btree![1, 2, 3];

        assert_eq!(in_order(&tree), vec![1, 2, 3]);
    }
}
