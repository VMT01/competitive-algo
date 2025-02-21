//! 2235. \[**Easy**\] [Add Two Integers](https://leetcode.com/problems/add-two-integers)
//!
//! - `Math`
//!
//! cargo test ::add_two_integers
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.32 MB | Beats      -%
//!
//! Use the addition operator to add two integers

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[test]
fn add_two_integers_case_1() {
    assert_eq!(Solution::sum(12, 5), 17);
}

#[test]
fn add_two_integers_case_2() {
    assert_eq!(Solution::sum(-10, 4), -6);
}
