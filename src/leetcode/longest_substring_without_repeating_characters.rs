//! 3. \[**Medium**\] [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters)
//!
//! - `Hash Table`
//! - `String`
//! - `Sliding Window`
//!
//! cargo test ::longest_substring_without_repeating_characters
//!
//! Runtime: 0ms    | Beats 100.00%
//! Memory : 2.33MB | Beats  65.45%
//!
//! For each character, checks if the character is already in the map. If it is, retrieves the last
//! known position of this character. The length of the current valid substring is calculated and
//! compared with the current maximum length. The character's new position plus one (to include the
//! current character in the next potential substring) is inserted into the map.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }

        use std::collections::BTreeMap;
        let mut map = BTreeMap::<u8, usize>::new();
        let (mut slicing_index, mut result) = (0, 1);

        for (index, byte) in s.bytes().enumerate() {
            let pos = *map.get(&byte).unwrap_or(&0);
            slicing_index = slicing_index.max(pos);

            result = result.max(index - slicing_index + 1);
            map.insert(byte, index + 1);
        }

        result as i32
    }
}

#[test]
fn longest_substring_without_repeating_characters_case_1() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}

#[test]
fn longest_substring_without_repeating_characters_case_2() {
    assert_eq!(
        Solution::length_of_longest_substring("bbbbb".to_string()),
        1
    );
}

#[test]
fn longest_substring_without_repeating_characters_case_3() {
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
}
