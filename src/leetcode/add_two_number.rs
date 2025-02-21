//! 2. \[**Medium**\] [Add Two Numbers](https://leetcode.com/problems/add-two-numbers)
//!
//! - `Linked List`
//! - `Math`
//! - `Recursion`
//!
//! cargo test ::add_two_numbers
//!
//! Runtime: 0ms    | Beats 100.00%
//! Memory : 2.28MB | Beats  97.92%
//!
//! Calculates the sum of the values of the current nodes from both lists.
//!
//! - If the sum is less than 10, creates a new node with this value and continues to add the rest of the list.
//! - If the sum is 10 or more, it creates a new node with the remainder when divided by 10 (`sum -
//!   10`) and passes a new `ListNode` with value 1 as the carry-over to the next addtion.

use crate::{ListNode, Solution};

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(v), None) | (None, Some(v)) => Some(v),
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val;
                let (val, carry) = if sum < 10 {
                    (sum, None)
                } else {
                    (sum - 10, Some(Box::new(ListNode::new(1))))
                };

                Some(Box::new(ListNode {
                    val,
                    next: Solution::add_two_numbers(
                        Solution::add_two_numbers(l1.next, carry),
                        l2.next,
                    ),
                }))
            }
        }
    }
}

#[test]
fn add_two_numbers_case_1() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let output = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 8, next: None })),
        })),
    }));

    assert_eq!(Solution::add_two_numbers(l1, l2), output);
}

#[test]
fn add_two_numbers_case_2() {
    let l1 = Some(Box::new(ListNode { val: 0, next: None }));
    let l2 = Some(Box::new(ListNode { val: 0, next: None }));
    let output = Some(Box::new(ListNode { val: 0, next: None }));

    assert_eq!(Solution::add_two_numbers(l1, l2), output);
}

#[test]
fn add_two_numbers_case_3() {
    let l1 = Some(Box::new(ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode { val: 9, next: None })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        })),
    }));
    let output = Some(Box::new(ListNode {
        val: 8,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode { val: 1, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));

    assert_eq!(Solution::add_two_numbers(l1, l2), output);
}
