// From: https://github.com/kavirajk/rope
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    weight: usize,
    left: Option<Box<Rope>>,
    right: Option<Box<Rope>>,
}

#[derive(Debug)]
pub struct Leaf {
    buf: Rc<String>,
    start: usize,
    end: usize,
}

impl Leaf {
    fn new(s: &str) -> Self {
        Self {
            buf: Rc::new(s.to_string()),
            start: 0,
            end: s.len() - 1,
        }
    }

    fn weight(&self) -> usize {
        self.end - self.start + 1
    }

    fn split(&self, offset: usize) -> (Leaf, Leaf) {
        if offset == 0 {
            return (Leaf::new(""), Leaf::new(&self.buf.as_ref().clone()));
        }

        if offset >= self.weight() {
            return (Leaf::new(&self.buf.as_ref().clone()), Leaf::new(""));
        }

        let (left, right) = self.buf.split_at(self.start + offset);
        ((Leaf::new(left)), Leaf::new(right))
    }

    fn report(&self, start: usize, end: usize) -> Option<String> {
        match start >= self.start && end <= self.end {
            true => Some(self.buf[start..end + 1].to_string()),
            false => None,
        }
    }
}

#[derive(Debug)]
pub enum Rope {
    Node(Node),
    Leaf(Leaf),
}

impl Rope {
    pub fn new(s: &str) -> Rope {
        Rope::Leaf(Leaf::new(s))
    }

    pub fn buf(&self) -> Option<&str> {
        match self {
            Rope::Node(_) => None,
            Rope::Leaf(leaf) => Some(&leaf.buf),
        }
    }

    pub fn index(&self, i: usize) -> Option<char> {
        match self {
            Rope::Node(node) => match i <= node.weight {
                true => node.left.as_ref()?.index(i),
                false => node.right.as_ref()?.index(i - node.weight),
            },
            Rope::Leaf(leaf) => leaf.buf.chars().nth(i),
        }
    }

    pub fn weight(&self) -> usize {
        match self {
            Rope::Node(node) => node.weight,
            Rope::Leaf(leaf) => leaf.weight(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Rope::Node(node) => {
                node.weight
                    + node
                        .right
                        .as_ref()
                        .expect("right node cannot be None")
                        .len()
            }
            Rope::Leaf(leaf) => leaf.weight(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, Rope::Leaf(_))
    }

    pub fn is_node(&self) -> bool {
        matches!(self, Rope::Node(_))
    }

    pub fn join(left: Box<Rope>, right: Box<Rope>) -> Rope {
        Rope::Node(Node {
            weight: left.len(),
            left: Some(left),
            right: Some(right),
        })
    }

    pub fn split(&mut self, offset: usize) -> (Rope, Rope) {
        match self {
            Rope::Leaf(leaf) => {
                let (l, r) = leaf.split(offset);
                (Rope::Leaf(l), Rope::Leaf(r))
            }
            Rope::Node(node) => {
                let w = node.weight;

                // < not <= because w - always length of the string (offset -1)
                if offset < w {
                    let (l, r) = node
                        .left
                        .as_mut()
                        .expect("left child cannot be empty")
                        .split(offset);
                    let r = Rope::join(
                        Box::new(r),
                        node.right.take().expect("right child cannot be empty"),
                    );
                    return (l, r);
                }

                let (l, r) = node
                    .right
                    .as_mut()
                    .expect("right child cannot be empty")
                    .split(offset - w);
                let l = Rope::join(
                    Box::new(l),
                    node.right.take().expect("left child cannot be empty"),
                );
                (l, r)
            }
        }
    }

    pub fn insert(&mut self, s: &str, offset: usize) -> Rope {
        let (l, r) = self.split(offset);

        let leaf = Rope::new(s);

        let tmp = Rope::join(Box::new(l), Box::new(leaf));

        Rope::join(Box::new(tmp), Box::new(r))
    }

    pub fn delete(&mut self, start: usize, end: usize) -> Rope {
        let (l, mut r) = self.split(start);

        let (_, r2) = r.split(end - start + 1);

        Rope::join(Box::new(l), Box::new(r2))
    }

    pub fn report(&self, start: usize, end: usize) -> Option<String> {
        match self {
            Rope::Leaf(leaf) => leaf.report(start, end),
            Rope::Node(node) => {
                let len = end - start + 1;
                if len <= node.weight {
                    return node.left.as_ref()?.report(start, end);
                }
                let l = node.left.as_ref()?.report(start, node.weight - 1)?;
                let r = node.right.as_ref()?.report(0, len - node.weight - 1)?;
                Some(l + &r)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_new() {
        let rope = Rope::new("Hello, World!");
        assert!(rope.is_leaf());
    }

    #[test]
    fn test_rope_index() {
        let rope = Rope::new("Hello, World!");
        assert_eq!(rope.index(1).unwrap(), 'e');
        assert_eq!(rope.index(0).unwrap(), 'H');
        assert_eq!(rope.index(3).unwrap(), 'l');
        assert_eq!(rope.index(12).unwrap(), '!');
    }

    #[test]
    fn test_rope_join() {
        let rope1 = Rope::new("Hello,");
        let rope2 = Rope::new(" World!");

        let rope = Rope::join(Box::new(rope1), Box::new(rope2));

        assert_eq!(rope.index(1).unwrap(), 'e');
        assert_eq!(rope.index(0).unwrap(), 'H');
        assert_eq!(rope.index(3).unwrap(), 'l');
        assert_eq!(rope.index(12).unwrap(), '!');
    }

    #[test]
    fn test_rope_split() {
        let mut rope = Rope::new("Hello, World!");
        let (left, right) = rope.split(5);
        assert_eq!(left.buf(), Some("Hello"));
        assert_eq!(right.buf(), Some(", World!"));
    }

    #[test]
    fn test_rope_report() {
        let mut rope = Rope::new("Hello, World!");

        assert_eq!(rope.report(1, 5).unwrap(), "ello,");

        let (left, right) = rope.split(5);
        assert_eq!(left.report(0, 4).unwrap(), "Hello");
        assert_eq!(right.report(0, 7).unwrap(), ", World!");
        assert_eq!(right.report(0, 8), None);
    }

    #[test]
    fn test_rope_insert() {
        let mut rope = Rope::new("Hello, World!");

        let rope = rope.insert(" Cruel", 6);

        assert_eq!(rope.report(0, 18).unwrap(), "Hello, Cruel World!");
    }

    #[test]
    fn test_rope_delete() {
        let mut rope = Rope::new("Hello, World!");
        rope = rope.delete(2, 4);
        assert_eq!(rope.report(0, 9).unwrap(), "He, World!");
    }
}
