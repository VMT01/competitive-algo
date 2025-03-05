//! 2579. \[**Medium**\] [Count Total Number of Colored Cells](https://leetcode.com/problems/count-total-number-of-colored-cells)
//!
//! - `Math`
//!
//! cargo test ::count_total_number_of_colored_cells
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.26 MB | Beats  33.33%
//!
//! F[i] = F[i-1] + 4 * (i - 1)
//!      = 2 * i * (i - 1) + 1

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        2 * n * (n - 1) + 1
    }
}

#[test]
fn count_total_number_of_colored_cells_case_1() {
    assert_eq!(Solution::colored_cells(1), 1);
}

#[test]
fn count_total_number_of_colored_cells_case_2() {
    assert_eq!(Solution::colored_cells(2), 5);
}
