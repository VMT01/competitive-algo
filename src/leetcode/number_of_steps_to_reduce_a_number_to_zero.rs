//! 1342. \[**Easy**\] [Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero)
//!
//! - `Math`
//! - `Bit Manipulation`
//!
//! cargo test ::number_of_steps_to_reduce_a_number_to_zero
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.33 MB | Beats      -%
//!
//! Counting bit 0 & 1. The total number of steps is calculated using the formula `(ones * 2 +
//! zeros).saturating_sub(1)`. This formula considers that each set bit requires two operations
//! (either division by 2 or substraction by 1), while unset bits require one operation.
//! `saturating_sub(1)` ensures that if the result underflows, it remains zero instead of wrapping
//! aroung.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let ones = num.count_ones();
        let zeros = 32 - num.leading_zeros() - ones;
        (ones * 2 + zeros).saturating_sub(1) as i32
    }
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_1() {
    assert_eq!(Solution::number_of_steps(14), 6);
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_2() {
    assert_eq!(Solution::number_of_steps(8), 4);
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_3() {
    assert_eq!(Solution::number_of_steps(123), 12);
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_4() {
    assert_eq!(Solution::number_of_steps(0), 0);
}
