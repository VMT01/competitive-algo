//! 2206. \[**Easy**\] [Divide Array Into Equal Pairs](https://leetcode.com/problems/divide-array-into-equal-pairs)
//!
//! - `Array`
//! - `Hash Table`
//! - `Bit Manipulation`
//! - `Counting`
//!
//! cargo test ::divide_array_into_equal_pairs
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.13 MB | Beats 100.00%
//!
//! Solution 1: Using Hash Set to keep track of number occurence in array.
//!
//! Solution 2: Using bit Manipulation. Since 1 <= num <= 500, which use 9 bit for representation.
//! We should XOR `num` and `(num + 1) << 10` to avoid mis-calculation. Image we have to compare
//! two sets, if the first one after xor is 0, there's no way the second one is 0 too.
//!
//! For example: [3, 5, 6] is not a valid array, but `3 ^ 5 ^ 6` = 0, which return true, wrong
//! answer. But if we use (3 ^ (4 << 10)) ^ (5 ^ (6 << 10)) ^ (6 ^ (7 << 10)) = 5120.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        nums.iter()
            .fold(0, |acc, &num| acc ^ num ^ ((num + 1) << 10))
            .eq(&0)
    }
}

#[test]
fn divide_array_into_equal_pairs_case_1() {
    assert!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
}

#[test]
fn divide_array_into_equal_pairs_case_2() {
    assert!(!Solution::divide_array(vec![1, 2, 3, 4]));
}

#[test]
fn divide_array_into_equal_pairs_case_3() {
    assert!(!Solution::divide_array(vec![3, 1, 5, 7]));
}

#[test]
fn divide_array_into_equal_pairs_case_4() {
    assert!(!Solution::divide_array(vec![
        9, 9, 19, 10, 9, 12, 2, 12, 3, 3, 11, 5, 8, 4, 13, 6, 2, 11, 9, 19, 11, 15, 9, 17, 15, 12,
        5, 14, 12, 16, 18, 16, 10, 3, 8, 9, 16, 20, 2, 4, 16, 12, 11, 14, 20, 16, 2, 18, 17, 20, 3,
        13, 16, 17, 1, 1, 11, 20, 20, 4
    ]));
}
