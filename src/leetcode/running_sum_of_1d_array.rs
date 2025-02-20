//! [Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array)
//! cargo test ::running_sum_of_1d_array
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.37 MB | Beats  35.29%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        nums.iter_mut().fold(0, |acc, val| {
            *val += acc;
            *val
        });
        nums
    }
}

#[test]
fn running_sum_of_1d_array_case_1() {
    assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10])
}

#[test]
fn running_sum_of_1d_array_case_2() {
    assert_eq!(
        Solution::running_sum(vec![1, 1, 1, 1, 1]),
        vec![1, 2, 3, 4, 5]
    )
}

#[test]
fn running_sum_of_1d_array_case_3() {
    assert_eq!(
        Solution::running_sum(vec![3, 1, 2, 10, 1]),
        vec![3, 4, 6, 16, 17]
    )
}
