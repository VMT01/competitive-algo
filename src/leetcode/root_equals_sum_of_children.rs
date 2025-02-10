//! [Root Equals Sum of Children](https://leetcode.com/problems/root-equals-sum-of-children)
//! cargo test ::root_equals_sum_of_children
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.23 MB | Beats  46.15%
//!
//! Vì Rust sử dụng smart pointer nên trước tiên cần phải unpack và borrow

use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

#[allow(dead_code)]
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.as_ref().unwrap().borrow();
        let left = root.left.as_ref().unwrap().borrow();
        let right = root.right.as_ref().unwrap().borrow();

        root.val == left.val + right.val
    }
}

#[test]
fn root_equals_sum_of_children_case_1() {
    assert!(Solution::check_tree(Some(Rc::new(RefCell::new(
        TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
        }
    )))));
}

#[test]
fn root_equals_sum_of_children_case_2() {
    assert!(!Solution::check_tree(Some(Rc::new(RefCell::new(
        TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
        }
    )))));
}
