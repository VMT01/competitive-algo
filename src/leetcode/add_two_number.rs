//! [Add Two Numbers](https://leetcode.com/problems/add-two-numbers)
//! cargo test ::add_two_numbers
//!
//! Runtime: 0ms    | Beats 100.00%
//! Memory : 2.28MB | Beats  97.92%
//!
//! Sử dụng đệ quy để tính toán từng chữ số. Nếu tổng hai chữ số hiện tại, ta sử dụng thêm một lần
//! đệ quy nữa để tính vào số tiếp theo.

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
