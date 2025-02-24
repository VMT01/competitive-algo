//! 707. \[**Medium**\] [Design Linked List](https://leetcode.com/problems/design-linked-list)
//!
//! - `Linked List`
//! - `Design`
//!
//! cargo test ::design_linked_list
//!
//! Runtime:
//! Memory :

use std::{mem::drop, ptr::null_mut};

struct ListNode {
    val: i32,
    next: *mut ListNode,
    prev: *mut ListNode,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: null_mut(),
            prev: null_mut(),
        }
    }
}

#[derive(Debug)]
struct MyLinkedList {
    len: i32,
    head: *mut ListNode,
    tail: *mut ListNode,
}

#[allow(dead_code)]
impl MyLinkedList {
    fn new() -> Self {
        Self {
            len: 0,
            head: null_mut(),
            tail: null_mut(),
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index >= self.len {
            return -1;
        }

        let mut target = self.head;

        unsafe {
            for _ in 0..index {
                target = (*target).next;
            }
            (*target).val
        }
    }

    fn add_at_head(&mut self, val: i32) {
        self.len += 1;

        let new_head = Box::into_raw(Box::new(ListNode::new(val)));
        unsafe {
            if self.head.is_null() {
                self.head = new_head;
                self.tail = new_head;
            } else {
                (*self.head).prev = new_head;
                (*new_head).next = self.head;
                self.head = new_head;
            }
        }
    }

    fn add_at_tail(&mut self, val: i32) {
        self.len += 1;

        let new_tail = Box::into_raw(Box::new(ListNode::new(val)));
        unsafe {
            if self.tail.is_null() {
                self.tail = new_tail;
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
                (*new_tail).prev = self.tail;
                self.tail = new_tail;
            }
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        match index {
            0 => self.add_at_head(val),
            index if index == self.len => self.add_at_tail(val),
            index if index > self.len => {}
            _ => {
                self.len += 1;

                let new_node = Box::into_raw(Box::new(ListNode::new(val)));
                let mut node = self.head;

                unsafe {
                    for _ in 0..index {
                        node = (*node).next;
                    }

                    (*(*node).prev).next = new_node;
                    (*new_node).next = node;
                    (*new_node).prev = (*node).prev;
                    (*node).prev = new_node;
                }
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        match (index, self.len) {
            (index, len) if index >= len => {}
            (0, 1) => {
                self.len -= 1;

                let target = self.head;
                self.head = null_mut();
                self.tail = null_mut();

                unsafe {
                    drop(Box::from_raw(target));
                }
            }
            (index, len) if index == len - 1 => {
                self.len -= 1;

                let target = self.tail;
                unsafe {
                    self.tail = (*target).prev;
                    (*self.tail).next = null_mut();

                    drop(Box::from_raw(target));
                }
            }
            (0, _) => {
                self.len -= 1;

                let target = self.head;
                unsafe {
                    self.head = (*target).next;
                    (*(*target).next).prev = null_mut();
                    drop(Box::from_raw(target));
                }
            }
            (index, _) => {
                self.len -= 1;

                let mut target = self.head;
                unsafe {
                    for _ in 0..index {
                        target = (*target).next;
                    }

                    (*(*target).prev).next = (*target).next;
                    (*(*target).next).prev = (*target).prev;
                    drop(Box::from_raw(target));
                }
            }
        }
    }
}

#[test]
fn design_linked_list_case_1() {
    let mut my_linked_list = MyLinkedList::new();
    my_linked_list.add_at_head(1);
    my_linked_list.add_at_tail(3);
    my_linked_list.add_at_index(1, 2);
    assert_eq!(my_linked_list.get(1), 2);
    my_linked_list.delete_at_index(1);
    assert_eq!(my_linked_list.get(1), 3);
}
