//! [Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array)
//! cargo test ::remove_duplicates_from_sorted_array
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.38 MB | Beats  87.84%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut jump_idx = 1;
        for idx in 1..nums.len() {
            if nums[idx] != nums[jump_idx - 1] {
                nums[jump_idx] = nums[idx];
                jump_idx += 1;
            }
        }

        jump_idx as i32
    }
}

#[test]
fn remove_duplicates_from_sorted_array_case_1() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    assert_eq!(nums[..2], vec![1, 2]);
}

#[test]
fn remove_duplicates_from_sorted_array_case_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
}
