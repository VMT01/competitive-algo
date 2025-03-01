//! 2460. \[**Easy**\] [Apply Operations to an Array](https://leetcode.com/problems/apply-operations-to-an-array)
//!
//! - `Array`
//! - `Two Pointers`
//! - `Simulation`
//!
//! cargo test ::apply_operations_to_an_array
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.28 MB | Beats  95.45%
//!
//! Loop over the vector and simulate the operation. After finish the simulation, we extend the
//! result vector with `nums.len() - result.len()` 0

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut result = Vec::with_capacity(len);
        let mut i = 0;

        while i < len - 1 {
            if nums[i] != 0 {
                if nums[i] == nums[i + 1] {
                    result.push(nums[i] * 2);
                    i += 1;
                } else {
                    result.push(nums[i]);
                }
            }
            i += 1;
        }
        if i < len && nums[i] != 0 {
            result.push(nums[i]);
        }

        result.extend(std::iter::repeat(0).take(len - result.len()));
        result
    }
}

#[test]
fn apply_operations_to_an_array_case_1() {
    assert_eq!(
        Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
        vec![1, 4, 2, 0, 0, 0]
    );
}

#[test]
fn apply_operations_to_an_array_case_2() {
    assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
}

#[test]
fn apply_operations_to_an_array_case_3() {
    assert_eq!(
        Solution::apply_operations(vec![
            847, 847, 0, 0, 0, 399, 416, 416, 879, 879, 206, 206, 206, 272
        ]),
        vec![1694, 399, 832, 1758, 412, 206, 272, 0, 0, 0, 0, 0, 0, 0]
    );
}
