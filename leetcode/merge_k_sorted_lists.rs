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

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

struct Solution;

impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 3.19MB |  77.54%
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());
        for list in lists.into_iter().flatten() {
            heap.push(list);
        }

        let mut root = Box::new(ListNode::new(0));
        let mut curr_node = &mut root;

        while let Some(node) = heap.pop() {
            let mut new_node = Box::new(ListNode::new(node.val));
            curr_node.next = Some(node);
            curr_node = curr_node.next.as_mut().unwrap();

            if let Some(next_node) = node.next {
                heap.push(next_node);
            }
        }

        root.next
    }
}

#[test]
fn test_merge_k_sorted_lists_1() {
    let lists = vec![
        Some(Box::new(
            ListNode::new(1).append(ListNode::new(4).append(ListNode::new(5))),
        )),
        Some(Box::new(
            ListNode::new(1).append(ListNode::new(3).append(ListNode::new(4))),
        )),
        Some(Box::new(ListNode::new(2).append(ListNode::new(6)))),
    ];

    let head =
        Some(Box::new(ListNode::new(1).append(
            ListNode::new(1).append(
                ListNode::new(2).append(
                    ListNode::new(3).append(ListNode::new(4).append(
                        ListNode::new(4).append(ListNode::new(5).append(ListNode::new(6))),
                    )),
                ),
            ),
        )));

    assert_eq!(head, Solution::merge_k_lists(lists));
}

#[test]
fn test_merge_k_sorted_lists_2() {
    let lists = vec![];
    let head = None;

    assert_eq!(head, Solution::merge_k_lists(lists));
}

#[test]
fn test_merge_k_sorted_lists_3() {
    let lists = vec![None];
    let head = None;

    assert_eq!(head, Solution::merge_k_lists(lists));
}
