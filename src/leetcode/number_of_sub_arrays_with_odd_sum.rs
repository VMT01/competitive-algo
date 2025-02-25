//! 1524. \[**Medium**\] [Number of Sub-arrays With Odd Sum](https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum)
//!
//! - `Array`
//! - `Math`
//! - `Dynamic Programming`
//! - `Prefix Sum`
//!
//! cargo test ::number_of_sub_arrays_with_odd_sum
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.92 MB | Beats 100.00%
//!
//! Use prefix sum. Calculate the sum of the array elements from the beginning to the current
//! element and track the even and odd occurrences of the sums.
//!
//! F[i] = F[i - 1] + sum % 2 ? odd : even
//!
//! Because:
//! - When current sum is even, which mean we have to subtract with other odd sum to get sub array
//!   with odd sum.
//! - The same thing as odd sum.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MODULO: i32 = 1_000_000_007;

        let mut prefix_sum = 0;
        let mut even_count = 1;
        let mut odd_count = 0;
        let mut result = 0;

        for num in arr {
            prefix_sum = (prefix_sum + num) & 1;

            if prefix_sum == 1 {
                result = (result + even_count) % MODULO;
                odd_count += 1;
            } else {
                result = (result + odd_count) % MODULO;
                even_count += 1;
            }
        }

        result
    }
}

#[test]
fn number_of_sub_arrays_with_odd_sum_case_1() {
    assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4)
}

#[test]
fn number_of_sub_arrays_with_odd_sum_case_2() {
    assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0)
}

#[test]
fn number_of_sub_arrays_with_odd_sum_case_3() {
    assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16)
}
