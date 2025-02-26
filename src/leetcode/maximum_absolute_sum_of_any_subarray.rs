//! 1749. \[**Medium**\] [Maximum Absolute Sum of Any Subarray](https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray)
//!
//! - `Array`
//! - `Dynamic Programming`
//!
//! cargo test ::maximum_absolute_sum_of_any_subarray
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.97 MB | Beats 100.00%
//!
//! result = max(prefix_sum) - min(prefix_sum)

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (max_sum, min_sum, _) =
            nums.iter()
                .fold((0, 0, 0), |(max_sum, min_sum, curr_sum), &num| {
                    let curr_sum = curr_sum + num;

                    (max_sum.max(curr_sum), min_sum.min(curr_sum), curr_sum)
                });

        max_sum - min_sum
    }
}

#[test]
fn maximum_absolute_sum_of_any_subarray_case_1() {
    assert_eq!(Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]), 5)
}

#[test]
fn maximum_absolute_sum_of_any_subarray_case_2() {
    assert_eq!(Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8)
}
