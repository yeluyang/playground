/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        sorted_array_to_bst_recursive(&nums)
    }
}

fn sorted_array_to_bst_recursive(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        None
    } else {
        let mid = nums.len() / 2;
        let (left, right) = nums.split_at(mid);

        let left = sorted_array_to_bst_recursive(left);

        let (root, right) = right.split_first().unwrap();
        let right = sorted_array_to_bst_recursive(right);

        let mut root = TreeNode::new(*root);
        root.left = left;
        root.right = right;

        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
