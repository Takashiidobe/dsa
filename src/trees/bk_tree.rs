// From the Bk_tree crate: https://github.com/IGI-111/bktree
use std::collections::VecDeque;

//@ A Bktree is a data structure for searching in discrete metric spaces. It can be used for
//@ approximate string matching, a.k.a. fuzzy searching.

//@ Every BkTree Node has a word and a set of children. The children also have a `usize` variable
//@ which show their distance from the Node.
struct Node<T> {
    word: T,
    children: Vec<(usize, Node<T>)>,
}

//@ A definition of a distance function.
pub type DistanceFn<T> = dyn Fn(&T, &T) -> usize;

//@ A Bktree has a root node and a distance function, which it uses to calculate distances between
//@ words.
pub struct BkTree<T> {
    root: Option<Box<Node<T>>>,
    dist: Box<DistanceFn<T>>,
}

impl<T> BkTree<T> {
    //@ creating a new BKTree requires just the distance function passed in.
    pub fn new(dist_fn: impl Fn(&T, &T) -> usize + 'static) -> Self {
        Self {
            root: None,
            dist: Box::new(dist_fn),
        }
    }

    //@ To insert a node
    pub fn insert(&mut self, val: T) {
        match &mut self.root {
            //@ If there is a root:
            Some(root) => {
                let mut root = &mut **root;
                loop {
                    //@ calculate the distance of root to this word we want to insert.
                    let k = (self.dist)(&root.word, &val);
                    //@ if the distance is 0, we can ignore this word since it is a duplicate.
                    if k == 0 {
                        return;
                    }

                    //@ find children with distance equal to k
                    let v = root.children.iter().position(|(dist, _)| *dist == k);
                    match v {
                        //@ if we found a match, we set the root to that position's node.
                        Some(pos) => {
                            root = &mut root.children[pos].1;
                        }
                        //@ if there are no matches, create a new child with distance k and this
                        //@ node
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
            //@ if there is no root, this node is the root node.
            None => {
                self.root = Some(Box::new(Node {
                    word: val,
                    children: vec![],
                }))
            }
        }
    }

    //@ To find a node, we require a value and a maximum distance to search for.
    pub fn find(&self, val: &T, max_dist: usize) -> Vec<(&T, usize)> {
        match self.root {
            //@ we search through the root
            Some(ref root) => {
                let mut matches = vec![];

                let mut candidates: VecDeque<&Node<T>> = VecDeque::new();

                candidates.push_back(root);

                //@ starting at the root, bfs through its children
                while let Some(n) = candidates.pop_front() {
                    //@ calculating each node's distance from the root
                    let distance = (self.dist)(&n.word, val);
                    //@ if the distance is less than the allowed distance, add it to the match
                    if distance <= max_dist {
                        matches.push((&n.word, distance));
                    }

                    //@ add any children of this node that have <= distance than this candidate.
                    candidates.extend(
                        n.children
                            .iter()
                            .filter(|(arc, _)| (arc.saturating_sub(distance) <= max_dist))
                            .map(|(_, node)| node),
                    );
                }

                matches
            }
            //@ If there are no nodes, there are no matches.
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

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
        let (words, dists): (Vec<&str>, Vec<usize>) = bk.find(&"bo", 2).into_iter().unzip();
        assert_eq!(words, ["book", "boo", "boon"]);
        assert_eq!(dists, [2, 1, 2]);
    }

    #[quickcheck]
    fn prop_bk_tree_search_correctness(
        words: Vec<String>,
        target: String,
        tolerance: usize,
    ) -> bool {
        if words.len() > 100 {
            return true;
        }
        if words.iter().any(|w| w.len() > 100) {
            return true;
        }
        let mut tree = BkTree::new(levenshtein_distance);
        for word in words.into_iter() {
            tree.insert(word);
        }
        let results = tree.find(&target, tolerance);
        results
            .iter()
            .all(|word| levenshtein_distance(word.0, &target) <= tolerance)
    }
}
