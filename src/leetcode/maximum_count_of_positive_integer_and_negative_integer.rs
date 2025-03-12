//! 2529. \[**Easy**\] [Maximum Count of Positive Integer and Negative Integer](https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer)
//!
//! - `Array`
//! - `Binary Search`
//! - `Counting`
//!
//! cargo test ::maximum_count_of_positive_integer_and_negative_integer
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.27 MB | Beats 100.00%
//!
//! Using binary search to find the position of `0` and `1`. Since position of `0` is equal to
//! negative count and `n - pos_1` is equal to positive count so we can get the result by compare
//! `pos_0` and `n - pos_1`.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    fn binary_search(nums: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if nums[0] > 0 || nums[n - 1] < 0 {
            return n as i32;
        }

        let (neg, pos) = (
            Self::binary_search(&nums, 0),
            n - Self::binary_search(&nums, 1),
        );

        std::cmp::max(neg, pos) as i32
    }
}

#[test]
fn maximum_count_of_positive_integer_and_negative_integer_case_1() {
    assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3)
}

#[test]
fn maximum_count_of_positive_integer_and_negative_integer_case_2() {
    assert_eq!(Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3)
}

#[test]
fn maximum_count_of_positive_integer_and_negative_integer_case_3() {
    assert_eq!(Solution::maximum_count(vec![5, 20, 66, 1314]), 4)
}
