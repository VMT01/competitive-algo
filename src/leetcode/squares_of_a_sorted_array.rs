//! 977. \[**Easy**\] [Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array)
//!
//! - `Array`
//! - `Two Pointers`
//! - `Sorting`
//!
//! cargo test ::squares_of_a_sorted_array
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.40 MB | Beats  98.85%
//!
//! Map each number to its square and collect into a vctor. Then, sorts this new vector in
//! ascending order.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.iter().map(|&num| num * num).collect::<Vec<_>>();
        result.sort_unstable();

        result
    }
}

#[test]
fn squares_of_a_sorted_array_case_1() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
}

#[test]
fn squares_of_a_sorted_array_case_2() {
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
