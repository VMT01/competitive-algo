//! [Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list)
//! cargo test ::middle_of_the_linked_list
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.18 MB | Beats 100.00%
//!
//! Sử dụng fast & slow pointer với fast nhanh gấp đôi slow, khi đó nếu fast bị null thì slow chính
//! là middle node của linked list.

use crate::{ListNode, Solution};

#[allow(dead_code)]
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref().and_then(|node| node.next.as_ref());

        while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
            slow = slow_node.next.as_ref();
            fast = fast_node.next.as_ref().and_then(|node| node.next.as_ref());
        }

        slow.cloned()
    }
}

#[test]
fn middle_of_the_linked_list_case_1() {
    let input = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        })),
    }));
    let output = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(5))),
        })),
    }));

    assert_eq!(Solution::middle_node(input), output);
}

#[test]
fn middle_of_the_linked_list_case_2() {
    let input = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode::new(6))),
                    })),
                })),
            })),
        })),
    }));
    let output = Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode::new(6))),
        })),
    }));

    assert_eq!(Solution::middle_node(input), output);
}
