//! 160. \[**Easy**\] [Intersection of Two Linked
//! Lists](https://leetcode.com/problems/intersection-of-two-linked-lists)
//!
//! - `Hash Table`
//! - `Linked List`
//! - `Two Pointers`
//!
//! Runtime: 51ms     | Beats 98.22%
//! Memory : 63.87 MB | Beats  7.47%
//!
//! If the lists intersect, the pointers will eventually meet at the intersection point. This works
//! because after switching lists, both pointers will have traveled the same total distance when
//! they met

class ListNode {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function getIntersectionNode(headA, headB) {
  let nodeA = headA,
    nodeB = headB;

  while (nodeA != nodeB) {
    nodeA = nodeA ? nodeA.next : headB;
    nodeB = nodeB ? nodeB.next : headA;
  }

  return nodeA;
}

(() => {
  const headA = new ListNode(1);
  headA.next = new ListNode(9);
  headA.next.next = new ListNode(1);

  const headB = new ListNode(3);

  const intersectionNode = new ListNode(2);
  intersectionNode.next = new ListNode(4);

  headA.next.next.next = intersectionNode;
  headB.next = intersectionNode;

  const node = getIntersectionNode(headA, headB);
  console.log(node);
})();
