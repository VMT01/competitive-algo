//! [Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree)
//! cargo test ::invert_binary_tree
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.22 MB | Beats  52.63%
//!
//! Sử dụng đệ quy và mem::swap để thay đổi pointer đến vùng nhớ của left & right

use std::{cell::RefCell, rc::Rc};

use crate::{Solution, TreeNode};

#[allow(dead_code)]
impl TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    fn swap_all(&mut self) {
        if let Some(node) = self.left.as_mut() {
            node.borrow_mut().swap_all();
        }
        if let Some(node) = self.right.as_mut() {
            node.borrow_mut().swap_all();
        }
        self.swap();
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.inspect(|node| node.borrow_mut().swap_all())
    }
}

#[test]
fn invert_binary_tree_case_1() {
    let input = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
    })));
    let output = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    })));
    assert_eq!(Solution::invert_tree(input), output);
}

#[test]
fn invert_binary_tree_case_2() {
    let input = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let output = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    })));
    assert_eq!(Solution::invert_tree(input), output);
}

#[test]
fn invert_binary_tree_case_3() {
    let input = None;
    let output = None;
    assert_eq!(Solution::invert_tree(input), output);
}
