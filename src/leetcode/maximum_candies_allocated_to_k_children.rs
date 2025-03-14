//! 2226. \[**Medium**\] [Maximum Candies Allocated to K Children](https://leetcode.com/problems/maximum-candies-allocated-to-k-children)
//!
//! - `Array`
//! - `Binary Search`
//!
//! cargo test ::maximum_candies_allocated_to_k_children
//!
//! Runtime: 17ms    | Beats  75.00%
//! Memory : 3.35 MB | Beats 100.00%
//!
//! Assum we want to give each child `m` candies, for each pile of `candies[i]` we can divide out
//! at most `candies[i] / m` sub piles with each pile `m` candies.
//! We can sum up all the sub piles then compare with the `k` children.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let total = candies.iter().fold(0, |acc, &candy| acc + candy as i64);
        if total < k {
            return 0;
        }

        let (mut left, mut right) = (0, *candies.iter().max().unwrap_or(&0));

        let can_allocate = |candy| {
            let mut sum = 0;
            for &size in &candies {
                sum += (size / candy) as i64;
            }
            k <= sum
        };

        while left < right {
            let mid = left + (right - left + 1) / 2;
            if can_allocate(mid) {
                left = mid
            } else {
                right = mid - 1;
            }
        }

        left
    }
}

#[test]
fn maximum_candies_allocated_to_k_children_case_1() {
    assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5);
}

#[test]
fn maximum_candies_allocated_to_k_children_case_2() {
    assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
}

#[test]
fn maximum_candies_allocated_to_k_children_case_3() {
    assert_eq!(Solution::maximum_candies(vec![4, 7, 5], 4), 3);
}
