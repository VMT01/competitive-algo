//! [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters)
//! cargo test ::longest_substring_without_repeating_characters
//!
//! Runtime: 0ms    | Beats 100.00%
//! Memory : 2.33MB | Beats  65.45%
//!
//! Sử dụng map để đánh dấu vị trí xuất hiện của các ký tự, kết hợp với slicing index để giả lập
//! two pointer. Kết quả là khoảng cách giữa con trỏ của chuỗi và slicing index. Việc insert vào
//! map phải cộng một để slicing index không nhầm lẫn với ký tự xuất hiện trước đó.

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
