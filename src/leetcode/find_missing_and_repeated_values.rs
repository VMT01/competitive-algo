//! 2965. \[**Easy**\] [Find Missing and Repeated Values](https://leetcode.com/problems/find-missing-and-repeated-values)
//!
//! - `Array`
//! - `Hash Table`
//! - `Math`
//! - `Matrix`
//!
//! cargo test ::find_missing_and_repeated_values
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.30 MB | Beats  32.14%
//!
//! Create a counter with n^2 elements to track which item is repeated or missing.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len().pow(2);
        let mut counter = vec![0; n + 1];

        for row in grid {
            for num in row {
                counter[num as usize] += 1u16;
            }
        }

        let (mut repeated, mut missing) = (0, 0);
        for (idx, count) in counter.iter().enumerate() {
            if 2.eq(count) {
                repeated = idx as i32;
            } else if 0.eq(count) {
                missing = idx as i32;
            }

            if repeated != 0 && missing != 0 {
                break;
            }
        }

        vec![repeated, missing]
    }
}

#[test]
fn find_missing_and_repeated_values_case_1() {
    assert_eq!(
        Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
        vec![2, 4]
    )
}

#[test]
fn find_missing_and_repeated_values_case_2() {
    assert_eq!(
        Solution::find_missing_and_repeated_values(vec![
            vec![9, 1, 7],
            vec![8, 9, 2],
            vec![3, 4, 6]
        ]),
        vec![9, 5]
    )
}

#[test]
fn find_missing_and_repeated_values_case_3() {
    assert_eq!(
        Solution::find_missing_and_repeated_values(vec![vec![2, 2], vec![3, 4]]),
        vec![2, 1]
    )
}
