//! [Remove Element](https://leetcode.com/problems/remove-element)
//! cargo test ::remove_element
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.25 MB | Beats  57.14%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut result = 0;
        for (i, num) in nums.clone().iter().enumerate().rev() {
            if val.eq(num) {
                nums.swap_remove(i);
            } else {
                result += 1;
            }
        }
        result
    }
}

#[test]
fn remove_element_case_1() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(Solution::remove_element(&mut nums, 3), 2);

    nums.sort();
    assert_eq!(nums, vec![2, 2]);
}

#[test]
fn remove_element_case_2() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(Solution::remove_element(&mut nums, 2), 5);

    nums.sort();
    assert_eq!(nums, vec![0, 0, 1, 3, 4]);
}
