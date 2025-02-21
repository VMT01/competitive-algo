//! 1089. \[**Easy**\] [Duplicate Zeros](https://leetcode.com/problems/duplicate-zeros)
//!
//! - `Array`
//! - `Two Pointers`
//!
//! cargo test ::duplicate_zeros
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.48 MB | Beats  18.03%
//!
//! For each elements in reverse, if the element is zero, it removes the last element from the
//! vector and inserts a new zero at the current index.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        for (i, num) in arr.clone().iter().enumerate().rev() {
            if 0.eq(num) {
                arr.pop();
                arr.insert(i, 0);
            }
        }
    }
}

#[test]
fn duplicate_zeros_case_1() {
    let mut input = vec![1, 0, 2, 3, 0, 4, 5, 0];
    let output = vec![1, 0, 0, 2, 3, 0, 0, 4];
    Solution::duplicate_zeros(&mut input);

    assert_eq!(input, output);
}

#[test]
fn duplicate_zeros_case_2() {
    let mut input = vec![1, 2, 3];
    let output = vec![1, 2, 3];
    Solution::duplicate_zeros(&mut input);

    assert_eq!(input, output);
}
