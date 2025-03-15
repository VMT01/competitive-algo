//! 2560. \[**Medium**\] [House Robber IV](https://leetcode.com/problems/house-robber-iv)
//!
//! - `Array`
//! - `Binary Search`
//!
//! cargo test ::house_robber_iv
//!
//! Runtime: 14ms   | Beats 100.00%
//! Memory : 3.90MB | Beats 100.00%
//!
//! Use binary search to find the smallest capability within the range of `nums::min` and
//! `nums::max`. For each trial value, use a greedy algorithm to select houses with values greater
//! than the current trial value. If the number of selected houses is greater than `k`, decrease
//! the maximum capability; otherwise, increase the minimum capability.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let (mut right, mut left) = nums.iter().fold((0, i32::MAX), |(max, min), &num| {
            (max.max(num), min.min(num))
        });

        let nums_len = nums.len();
        let can_take_k_houses = |capability: i32| -> bool {
            let (mut index, mut count) = (0, 0);
            while index < nums_len {
                if nums[index] <= capability {
                    count += 1;
                    index += 2;
                } else {
                    index += 1
                }

                if count >= k {
                    return true;
                }
            }

            false
        };

        while left < right {
            let mid = left + ((right - left) >> 1);

            if can_take_k_houses(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

#[test]
fn house_robber_iv_case_1() {
    assert_eq!(Solution::min_capability(vec![2, 3, 5, 9], 2), 5)
}

#[test]
fn house_robber_iv_case_2() {
    assert_eq!(Solution::min_capability(vec![2, 7, 9, 3, 1], 2), 2)
}
