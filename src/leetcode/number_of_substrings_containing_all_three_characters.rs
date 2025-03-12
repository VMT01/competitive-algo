//! 1358. \[**Medium**\] [Number of Substrings Containing All Three Characters](https://leetcode.com/problems/number-of-substrings-containing-all-three-characters)
//!
//! - `Hash Table`
//! - `String`
//! - `Sliding Window`
//!
//! cargo test ::number_of_substrings_containing_all_three_characters
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.34 MB | Beats  62.50%
//!
//! If `a`, `b` and `c` are presented, all possible substrings will be started from 0 to the index
//! of the first occured character (min(a, b, c)) and ended at current index posisition.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        s.char_indices()
            .fold((-1, -1, -1, 0), |(a, b, c, m), (i, ch)| {
                let i = i as i32;
                match ch {
                    'a' => (i, b, c, m + i.min(b).min(c) + 1),
                    'b' => (a, i, c, m + i.min(a).min(c) + 1),
                    'c' => (a, b, i, m + i.min(a).min(b) + 1),
                    _ => unreachable!(),
                }
            })
            .3
    }
}

#[test]
fn number_of_substrings_containing_all_three_characters_case_1() {
    assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
}

#[test]
fn number_of_substrings_containing_all_three_characters_case_2() {
    assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3);
}

#[test]
fn number_of_substrings_containing_all_three_characters_case_3() {
    assert_eq!(Solution::number_of_substrings("abc".to_string()), 1);
}
