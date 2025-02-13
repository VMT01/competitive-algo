//! [Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits)
//! cargo test ::find_numbers_with_even_number_of_digits
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory:  2.26 MB | Beats  95.16%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter_map(|num| (num.to_string().len() % 2 == 0).then_some(1))
            .sum()
    }
}

#[test]
fn find_numbers_with_even_number_of_digits_case_1() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
}

#[test]
fn find_numbers_with_even_number_of_digits_case_2() {
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
}
