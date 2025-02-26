//! 142. \[**Medium**\] [Linked List Cycle
//! II](https://leetcode.com/problems/linked-list-cycle-ii)
//!
//! - `Hash Table`
//! - `Linked List`
//! - `Two Pointers`
//!
//! Runtime: 4ms      | Beats 87.32%
//! Memory : 10.28 MB | Beats 95.54%
//!
//! Using two pointers, where fast pointer is 2 time faster than slow pointer.
//! If slow == fast,
//! which mean there a cycle in the linked list.
//!
//! At that point, we have some declaration
//!  - x: distance from `head` to `cycle start point`
//!  - y: distance from `cycle start point` to `slow & fast met point`
//!  - slow: moved `x + y` steps
//!  - fast: moved `2 * (x + y)` steps
//!
//! We have:
//! > fast steps - slow step = circle length * N
//! > or
//! > 2 * (x + y) - (x + y) = C * N
//! > equivalent to
//! > x + y = C * N
//!
//! So, from current slow pointer, need to step more x steps to reach the cycle
//! start point, which equivalent to the step of head step to cycle start point.

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

struct ListNode {
  int val;
  struct ListNode *next;
};

struct ListNode *detectCycle(struct ListNode *head) {
  if (head == NULL) {
    return NULL;
  }

  struct ListNode *fast = head, *slow = head;
  while (fast && fast->next) {
    fast = fast->next->next;
    slow = slow->next;

    if (fast == slow) {
      break;
    }
  }

  if (!(fast && fast->next)) {
    return NULL;
  }

  while (head != slow) {
    head = head->next;
    slow = slow->next;
  }

  return head;
}

int main() {
  struct ListNode node1, node2, node3, node4;

  node1.val = 3, node1.next = &node2;
  node2.val = 2, node2.next = &node3;
  node3.val = 0, node3.next = &node4;
  node4.val = -4, node4.next = &node2;

  printf("Node 1: %p - Value: %d - Next: %p\n", &node1, node1.val, node1.next);
  printf("Node 2: %p - Value: %d - Next: %p\n", &node2, node2.val, node2.next);
  printf("Node 3: %p - Value: %d - Next: %p\n", &node3, node3.val, node3.next);
  printf("Node 4: %p - Value: %d - Next: %p\n\n", &node4, node4.val,
         node4.next);

  assert(detectCycle(&node1) == &node2);

  return 0;
}
