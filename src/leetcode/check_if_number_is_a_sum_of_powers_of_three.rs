//! 1780. \[**Medium**\] [Check if Number is a Sum of Powers of Three](https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three)
//!
//! - `Math`
//!
//! cargo test ::check_if_number_is_a_sum_of_powers_of_three
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.22 MB | Beats  52.94%
//!
//! Greedy increase power upto `n` but not exceed `n`.
//! Always find highest power below n.
//!
//! ---
//!
//! Or there's another way. When we present n in ternary form, there only 3 digits are `0`, `1` and
//! `2` with these meaning:
//! - `0`: Not use this power
//! - `1`: Use this power one time
//! - `2`: Use this power TWO times
//!
//! Since use any power of three two times is invalid so we can use this condition to filter out
//! correct number

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}

#[test]
fn check_if_number_is_a_sum_of_powers_of_three_case_1() {
    assert!(Solution::check_powers_of_three(12))
}

#[test]
fn check_if_number_is_a_sum_of_powers_of_three_case_2() {
    assert!(Solution::check_powers_of_three(91))
}

#[test]
fn check_if_number_is_a_sum_of_powers_of_three_case_3() {
    assert!(!Solution::check_powers_of_three(21))
}
