//! 485. \[**Easy**\] [Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones)
//!
//! - `Array`
//!
//! cargo test ::max_consecutive_ones
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.68 MB | Beats  86.78%
//!
//! For each element in the iterator. if the element is zero, it means a streak of consecutive ones
//! has ended, updates `result` to be the maximum of its current value and the current counter,
//! then reset the counter.

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
