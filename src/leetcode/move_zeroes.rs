//! [Move Zeroes](https://leetcode.com/problems/move-zeroes)
//! cargo test ::move_zeroes
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.43 MB | Beats  49.48%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let mut non_zero_idx = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(non_zero_idx, i);
                non_zero_idx += 1;
            }
        }
    }
}

#[test]
fn move_zeroes_case_1() {
    let mut input = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut input);

    assert_eq!(input, vec![1, 3, 12, 0, 0]);
}

#[test]
fn move_zeroes_case_2() {
    let mut input = vec![0];
    Solution::move_zeroes(&mut input);

    assert_eq!(input, vec![0]);
}

#[test]
fn move_zeroes_case_3() {
    let mut input = vec![1, 0, 1];
    Solution::move_zeroes(&mut input);

    assert_eq!(input, vec![1, 1, 0]);
}
