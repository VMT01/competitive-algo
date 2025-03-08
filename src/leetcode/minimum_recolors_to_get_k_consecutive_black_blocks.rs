//! 2379. \[**Easy**\] [Minimum Recolors to Get K Consecutive Black Blocks](https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks)
//!
//! - `String`
//! - `Sliding Window`
//!
//! cargo test ::minimum_recolors_to_get_k_consecutive_black_blocks
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.17 MB | Beats 100.00%
//!
//! Using sliding window and count for white blocks in that window. That is the number of
//! operations needed to recolor a consecutive black blocks.
//! The result is the minimum operations.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let k = k as usize;

        let mut white_count = blocks.iter().take(k).filter(|&byte| b'W'.eq(byte)).count();
        let mut result = white_count;

        for i in k..blocks.len() {
            if blocks[i - k] == b'W' {
                white_count -= 1;
            }
            if blocks[i] == b'W' {
                white_count += 1;
            }
            result = result.min(white_count);
        }

        result as i32
    }
}

#[test]
fn minimum_recolors_to_get_k_consecutive_black_blocks_case_1() {
    assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
}

#[test]
fn minimum_recolors_to_get_k_consecutive_black_blocks_case_2() {
    assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
}
