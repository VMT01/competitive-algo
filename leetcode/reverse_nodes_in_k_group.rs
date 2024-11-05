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
    /// Memory : 2.36MB |  77.59%
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut node = &mut head;
        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return head;
            }
        }

        let mut res = Self::reverse_k_group(node.take(), k);
        while let Some(h) = head.take() {
            res = Some(Box::new(ListNode {
                val: h.val,
                next: res,
            }));
            head = h.next;
        }

        res
    }
}

#[test]
fn test_reverse_k_group_1() {
    let head = Some(Box::new(ListNode::new(1).append(ListNode::new(2).append(
        ListNode::new(3).append(ListNode::new(4).append(ListNode::new(5))),
    ))));
    let new_head = Some(Box::new(ListNode::new(2).append(ListNode::new(1).append(
        ListNode::new(4).append(ListNode::new(3).append(ListNode::new(5))),
    ))));
    assert_eq!(new_head, Solution::reverse_k_group(head, 2));
}

#[test]
fn test_reverse_k_group_2() {
    let head = Some(Box::new(ListNode::new(1).append(ListNode::new(2).append(
        ListNode::new(3).append(ListNode::new(4).append(ListNode::new(5))),
    ))));
    let new_head = Some(Box::new(ListNode::new(3).append(ListNode::new(2).append(
        ListNode::new(1).append(ListNode::new(4).append(ListNode::new(5))),
    ))));
    assert_eq!(new_head, Solution::reverse_k_group(head, 3));
}
