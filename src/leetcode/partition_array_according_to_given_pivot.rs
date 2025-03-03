//! 2161. \[**Medium**\] [Partition Array According to Given Pivot](https://leetcode.com/problems/partition-array-according-to-given-pivot)
//!
//! - `Array`
//! - `Two Pointers`
//! - `Simulation`
//!
//! cargo test ::partition_array_according_to_given_pivot
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 4.10 MB | Beats  63.64%
//!
//! Partition vector based on pivot as usual.
//! Then concat new vector using extend.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let (mut less, mut greater) = (vec![], vec![]);
        let mut pivot_count = 0;

        for &num in nums.iter() {
            match num.cmp(&pivot) {
                std::cmp::Ordering::Less => less.push(num),
                std::cmp::Ordering::Equal => pivot_count += 1,
                std::cmp::Ordering::Greater => greater.push(num),
            };
        }

        let mut result = Vec::with_capacity(nums.len());
        result.extend(&less);
        result.extend(std::iter::repeat(pivot).take(pivot_count));
        result.extend(&greater);

        result
    }
}

#[test]
fn partition_array_according_to_given_pivot_case_1() {
    assert_eq!(
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
        vec![9, 5, 3, 10, 10, 12, 14]
    )
}

#[test]
fn partition_array_according_to_given_pivot_case_2() {
    assert_eq!(
        Solution::pivot_array(vec![-3, 4, 3, 2], 2),
        vec![-3, 2, 4, 3]
    )
}
