//! 448. \[**Easy**\] [Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array)
//!
//! - `Array`
//! - `Hash Table`
//!
//! cargo test ::find_all_numbers_disappeared_in_an_array
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 3.80 MB | Beats  43.10%
//!
//! Marking presence by flipping the sign of numbers in the original array.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for num in nums.clone().into_iter() {
            if nums[(num - 1) as usize] > 0 {
                nums[(num - 1) as usize] *= -1;
            }
        }

        let mut result = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if num > 0 {
                result.push((i + 1) as i32);
            }
        }
        result
    }
}

#[test]
fn find_all_numbers_disappeared_in_an_array_case_1() {
    assert_eq!(
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
}

#[test]
fn find_all_numbers_disappeared_in_an_array_case_2() {
    assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
}
