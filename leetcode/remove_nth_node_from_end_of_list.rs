#[derive(Debug, Clone, PartialEq, Eq)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn append(&mut self, node: ListNode) -> Self {
        self.next = Some(Box::new(node));
        self.to_owned()
    }
}

struct Solution;

impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.10MB |  16.22%
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn recursive(mut node: Box<ListNode>, n: i32) -> (Option<Box<ListNode>>, Option<i32>) {
            let Some(next) = node.next else {
                if n == 1 {
                    return (None, None);
                }
                return (Some(node), Some(2));
            };

            match recursive(next, n) {
                (next, None) => {
                    node.next = next;
                    (Some(node), None)
                }
                (next, Some(ord)) if ord == n => (next, None),
                (next, Some(ord)) => {
                    node.next = next;
                    (Some(node), Some(ord + 1))
                }
            }
        }

        recursive(head?, n).0
    }
}

#[test]
fn test_remove_nth_from_end_1() {
    let head = Some(Box::new(ListNode::new(1).append(ListNode::new(2).append(
        ListNode::new(3).append(ListNode::new(4).append(ListNode::new(5))),
    ))));
    let new_head = Some(Box::new(
        ListNode::new(1).append(ListNode::new(2).append(ListNode::new(3).append(ListNode::new(5)))),
    ));
    assert_eq!(Solution::remove_nth_from_end(head, 2), new_head);
}

#[test]
fn test_remove_nth_from_end_2() {
    let head = Some(Box::new(ListNode::new(1)));
    assert_eq!(Solution::remove_nth_from_end(head, 1), None);
}

#[test]
fn test_remove_nth_from_end_3() {
    let head = Some(Box::new(ListNode::new(1).append(ListNode::new(2))));
    let new_head = Some(Box::new(ListNode::new(1)));
    assert_eq!(Solution::remove_nth_from_end(head, 1), new_head);
}
