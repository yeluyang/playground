use std::{cell::RefCell, collections::VecDeque, rc::Rc};

extern crate log;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

pub fn binary_tree_from(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if v.is_empty() {
        None
    } else {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))));
        let mut que = VecDeque::new();
        que.push_back(root.clone());
        let mut i = 0;
        while let Some(node) = que.pop_front() {
            if let Some(node) = node {
                if 2 * i + 1 < v.len() {
                    if let Some(val) = v[2 * i + 1] {
                        let left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        node.borrow_mut().left = left.clone();
                        que.push_back(left);
                    } else {
                        que.push_back(None);
                    }
                }
                if 2 * i + 2 < v.len() {
                    if let Some(val) = v[2 * i + 2] {
                        let right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        node.borrow_mut().right = right.clone();
                        que.push_back(right);
                    } else {
                        que.push_back(None);
                    }
                }
            }
            i += 1;
        }
        log::trace!("tree={:?}", root);
        root
    }
}
