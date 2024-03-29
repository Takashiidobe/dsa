#[derive(Default)]
pub enum Heap<T> {
    #[default]
    Empty,
    Item(Box<Node<T>>),
}

pub struct Node<T> {
    rank: usize,
    item: T,
    left: Heap<T>,
    right: Heap<T>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node {
            rank: 1,
            item,
            left: Heap::default(),
            right: Heap::default(),
        }
    }
}

impl<T> Heap<T> {
    fn rank(&self) -> usize {
        match self {
            Heap::Empty => 0,
            Heap::Item(item) => item.rank,
        }
    }
}

impl<T: PartialOrd> Heap<T> {
    fn make(item: T, left: Heap<T>, right: Heap<T>) -> Self {
        Heap::Item(Box::new(if left.rank() >= right.rank() {
            Node {
                rank: right.rank() + 1,
                item,
                left,
                right,
            }
        } else {
            Node {
                rank: left.rank() + 1,
                item,
                left: right,
                right: left,
            }
        }))
    }

    fn merge(a: Heap<T>, b: Heap<T>) -> Self {
        match (a, b) {
            (Heap::Empty, h) | (h, Heap::Empty) => h,
            (Heap::Item(h1), Heap::Item(h2)) => {
                if h1.item <= h2.item {
                    Heap::make(h1.item, h1.left, Heap::merge(h1.right, Heap::Item(h2)))
                } else {
                    Heap::make(h2.item, h2.left, Heap::merge(Heap::Item(h1), h2.right))
                }
            }
        }
    }

    pub fn insert(self, item: T) -> Self {
        let new_node = Box::new(Node::new(item));
        Heap::merge(Heap::Item(new_node), self)
    }

    pub fn find_min(&self) -> Option<&T> {
        match self {
            Heap::Empty => None,
            Heap::Item(node) => Some(&node.item),
        }
    }

    pub fn delete_min(self) -> Self {
        match self {
            Heap::Empty => self,
            Heap::Item(node) => Heap::merge(node.left, node.right),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn children_swap() {
        let mut heap = Heap::Empty;

        //    5
        //   /
        // 10
        heap = heap.insert(5).insert(10);
        if let Heap::Item(ref node) = heap {
            assert_eq!(node.left.find_min(), Some(&10));
            assert_eq!(node.right.find_min(), None);
        };

        //       5
        //      / \
        //    15   10
        //   /  \
        //  20  25
        heap = heap.insert(15).insert(20).insert(25);
        if let Heap::Item(ref node) = heap {
            assert_eq!(node.left.find_min(), Some(&15));
            assert_eq!(node.right.find_min(), Some(&10));
        }
    }
}
