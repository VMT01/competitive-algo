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
    /// Memory : 2.05MB |  86.76%
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut node = head.as_mut().unwrap().next.take();
        head.as_mut().unwrap().next = Self::swap_pairs(node.as_mut().unwrap().next.take());
        node.as_mut().unwrap().next = head;

        node
    }
}

#[test]
fn test_swap_nodes_in_pairs_1() {
    let head = Some(Box::new(ListNode::new(1).append(
        ListNode::new(2).append(ListNode::new(3).append(ListNode::new(4))),
    )));
    let new_head = Some(Box::new(
        ListNode::new(2).append(ListNode::new(1).append(ListNode::new(4).append(ListNode::new(3)))),
    ));

    assert_eq!(new_head, Solution::swap_pairs(head));
}

#[test]
fn test_swap_nodes_in_pairs_2() {
    let head = None;
    let new_head = None;

    assert_eq!(new_head, Solution::swap_pairs(head));
}

#[test]
fn test_swap_nodes_in_pairs_3() {
    let head = Some(Box::new(ListNode::new(1)));
    let new_head = Some(Box::new(ListNode::new(1)));

    assert_eq!(new_head, Solution::swap_pairs(head));
}

#[test]
fn test_swap_nodes_in_pairs_4() {
    let head = Some(Box::new(
        ListNode::new(1).append(ListNode::new(2).append(ListNode::new(3))),
    ));
    let new_head = Some(Box::new(
        ListNode::new(2).append(ListNode::new(1).append(ListNode::new(3))),
    ));

    assert_eq!(new_head, Solution::swap_pairs(head));
}
