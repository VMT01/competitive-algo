//! 1051. \[**Easy**\] [Height Checker](https://leetcode.com/problems/height-checker)
//!
//! - `Array`
//! - `Sorting`
//! - `Counting Sourt`
//!
//! cargo test ::height_checker
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.25 MB | Beats  64.00%
//!
//! Iterates through the original heights, using the count array to track of each student is
//! standing in the correct order and counting mismatches accordingly.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let max_height = *heights.iter().max().unwrap() as usize;

        let mut count = vec![0; max_height + 1];
        for &height in &heights {
            count[height as usize] += 1;
        }

        let mut mismatches = 0;
        let mut expected_height = 1;

        for &height in &heights {
            while expected_height <= max_height && count[expected_height] == 0 {
                expected_height += 1;
            }

            if height as usize != expected_height {
                mismatches += 1;
            }

            count[expected_height] -= 1;
        }

        mismatches
    }
}

#[test]
fn height_checker_case_1() {
    assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
}

#[test]
fn height_checker_case_2() {
    assert_eq!(Solution::height_checker(vec![5, 1, 2, 3, 4]), 5);
}

#[test]
fn height_checker_case_3() {
    assert_eq!(Solution::height_checker(vec![1, 2, 3, 4, 5]), 0);
}
