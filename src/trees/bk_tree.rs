// From the Bk_tree crate: https://github.com/IGI-111/bktree
use std::collections::VecDeque;

struct Node<T> {
    word: T,
    children: Vec<(usize, Node<T>)>,
}

pub type DistanceFn<T> = dyn Fn(&T, &T) -> usize;

pub struct BkTree<T> {
    root: Option<Box<Node<T>>>,
    dist: Box<DistanceFn<T>>,
}

impl<T> BkTree<T> {
    pub fn new(dist_fn: impl Fn(&T, &T) -> usize + 'static) -> Self {
        Self {
            root: None,
            dist: Box::new(dist_fn),
        }
    }

    pub fn insert(&mut self, val: T) {
        match &mut self.root {
            Some(root) => {
                let mut root = &mut **root;
                loop {
                    let k = (self.dist)(&root.word, &val);
                    // distance of 0 = same word
                    if k == 0 {
                        return;
                    }

                    // find children with distance equal to k
                    let v = root.children.iter().position(|(dist, _)| *dist == k);
                    match v {
                        // if we found a match, we set the root to that position's node.
                        Some(pos) => {
                            root = &mut root.children[pos].1;
                        }
                        // if there are no matches, create a new child with distance k and this
                        // node
                        None => {
                            root.children.push((
                                k,
                                Node {
                                    word: val,
                                    children: vec![],
                                },
                            ));
                            break;
                        }
                    }
                }
            }
            // if the root does not exist, this node is the root.
            None => {
                self.root = Some(Box::new(Node {
                    word: val,
                    children: vec![],
                }))
            }
        }
    }

    pub fn find(&self, val: T, max_dist: usize) -> Vec<(&T, usize)> {
        match self.root {
            Some(ref root) => {
                let mut matches = vec![];

                let mut candidates: VecDeque<&Node<T>> = VecDeque::new();

                candidates.push_back(root);

                // starting at the root, bfs through its children
                while let Some(n) = candidates.pop_front() {
                    let distance = (self.dist)(&n.word, &val);
                    // if the distance matches, add it to the match
                    if distance <= max_dist {
                        matches.push((&n.word, distance));
                    }

                    // add any children that have <= distance than this candidate.
                    candidates.extend(
                        n.children
                            .iter()
                            .filter(|(arc, _)| (arc.saturating_sub(distance) <= max_dist))
                            .map(|(_, node)| node),
                    );
                }

                matches
            }
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distances::levenshtein::levenshtein_distance;

    #[test]
    fn levenshtein_distance_test() {
        let mut bk = BkTree::new(levenshtein_distance);
        let words = vec![
            "book", "books", "boo", "boon", "cook", "cake", "cape", "cart",
        ];
        for word in words {
            bk.insert(word);
        }
        let (words, dists): (Vec<&str>, Vec<usize>) = bk.find("bo", 2).into_iter().unzip();
        assert_eq!(words, ["book", "boo", "boon"]);
        assert_eq!(dists, [2, 1, 2]);
    }
}
