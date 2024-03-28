use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub type BSTNode = Option<Rc<RefCell<TreeNode>>>;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct TreeNode {
    pub val: i32,
    pub left: BSTNode,
    pub right: BSTNode,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[macro_export]
macro_rules! btree{
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            use std::rc::Rc;
            use std::cell::RefCell;

            let elems = vec![$(stringify!($e)), *];
            let elems = elems.iter().map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>();
            let head = Some(Rc::new(RefCell::new(TreeNode::new(elems[0].unwrap()))));
            let mut nodes = std::collections::VecDeque::new();
            nodes.push_back(head.as_ref().unwrap().clone());

            for i in elems[1..].chunks(2) {
                let node = nodes.pop_front().unwrap();
                if let Some(val) = i[0]{
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if i.len() > 1 {
                    if let Some(val) = i[1] {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                    }
                }
            }
            head
        }
    };
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = vec![self.val];
        let mut p = self;
        while let Some(next) = p.next.as_ref() {
            v.push(next.val);
            p = next;
        }
        write!(f, "{:?}", v)
    }
}

type LLNode = Option<Box<ListNode>>;

#[macro_export]
macro_rules! slist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new($crate::ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new($crate::ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head;
            head.next
        }
    };
}

#[macro_export]
macro_rules! vec_string {
    ($($e:expr), *) => {vec![$($e.to_owned()), *]};
}

#[derive(Clone, Default)]
pub struct GraphNode {
    val: i32,
    neighbors: Vec<Rc<GraphNode>>,
}
