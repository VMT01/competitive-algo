//! 414. \[**Easy**\] [Third Maximum Number](https://leetcode.com/problems/third-maximum-number)
//!
//! - `Array`
//! - `Sorting`
//!
//! cargo test ::third_maximum_number
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.34 MB | Beats  58.82%
//!
//! Sort the vector in decreasing order and remove duplicates.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.dedup();

        nums[if nums.len() > 2 { 2 } else { 0 }]
    }
}

#[test]
fn third_maximum_number_case_1() {
    assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
}

#[test]
fn third_maximum_number_case_2() {
    assert_eq!(Solution::third_max(vec![1, 2]), 2);
}

#[test]
fn third_maximum_number_case_3() {
    assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
}
