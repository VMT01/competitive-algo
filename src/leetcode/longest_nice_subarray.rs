//! 2401. \[**Medium**\] [Longest Nice Subarray](https://leetcode.com/problems/longest-nice-subarray)
//!
//! - `Array`
//! - `Bit Manipulation`
//! - `Sliding Window`
//!
//! cargo test ::longest_nice_subarray
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 4.08 MB | Beats  50.00%
//!
//! Maintain a variable `used_bits` that tracks which bits are currently "used" by elements in our
//! window. For each element, if it doesn't share bit with `used_bit`, add to our window.
//! Otherwise, shrink the window from the left until there's no conflict.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left, mut right, mut result) = (0, 0, 1);
        let mut used_bit = 0;

        while right < n {
            while (used_bit & nums[right]) != 0 {
                used_bit ^= nums[left];
                left += 1;
            }
            used_bit |= nums[right];
            result = result.max(right - left + 1);
            right += 1;
        }

        result as i32
    }
}

#[test]
fn longest_nice_subarray_case_1() {
    assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3)
}

#[test]
fn longest_nice_subarray_case_2() {
    assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1)
}
