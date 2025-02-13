//! [Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones)
//! cargo test ::max_consecutive_ones
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.68 MB | Beats  86.78%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .chain(std::iter::once(&0))
            .fold((0, 0), |(result, counter), num| {
                if 0.eq(num) {
                    (result.max(counter), 0)
                } else {
                    (result, counter + 1)
                }
            })
            .0
    }
}

#[test]
fn max_consecutive_ones_case_1() {
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    );
}

#[test]
fn max_consecutive_ones_case_2() {
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
        2
    );
}
