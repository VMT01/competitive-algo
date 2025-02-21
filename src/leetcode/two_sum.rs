//! 1. \[**Easy**\] [Two Sum](https://leetcode.com/problems/two-sum)
//!
//! - `Array`
//! - `Hash Table`
//!
//! cargo test ::two_sum
//!
//! Runtime: 1ms     | Beats 49.00%
//! Memory : 2.47 MB | Beats 47.97%
//!
//! For each element, calculates the complement and checks if it exists in the map.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::BTreeMap::<i32, usize>::new();
        let mut result = Vec::with_capacity(2);

        for (index, value) in nums.into_iter().enumerate() {
            let complement = target - value;
            if map.contains_key(&complement) {
                result.push(map[&complement] as i32);
                result.push(index as i32);
                break;
            }
            map.insert(value, index);
        }

        result
    }
}

#[test]
fn two_sum_case_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn two_sum_case_2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn two_sum_case_3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
