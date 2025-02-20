//! [Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity)
//! cargo test ::sort_array_by_parity
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.39 MB | Beats  68.00%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd): (Vec<_>, Vec<_>) = nums.iter().partition(|&num| num % 2 == 0);

        even.sort_unstable();
        odd.sort_unstable();

        even.append(&mut odd);
        even
    }
}

#[test]
fn sort_array_by_parity_case_1() {
    assert_eq!(
        Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
        vec![2, 4, 1, 3]
    );
}

#[test]
fn sort_array_by_parity_case_2() {
    assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
}
