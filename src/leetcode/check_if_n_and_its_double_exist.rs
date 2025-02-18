//! [Check If N and Its Double Exist](https://leetcode.com/problems/check-if-n-and-its-double-exist)
//! cargo test ::check_if_n_and_its_double_exist
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.36 MB | Beats  44.12%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut nums = std::collections::HashSet::new();

        for num in arr.iter() {
            if nums.contains(&(num << 1)) || (num % 2 == 0 && nums.contains(&(num >> 1))) {
                return true;
            };
            nums.insert(num);
        }

        false
    }
}

#[test]
fn check_if_n_and_its_double_exist_case_1() {
    assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
}

#[test]
fn check_if_n_and_its_double_exist_case_2() {
    assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
}
