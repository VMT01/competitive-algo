//! [Ransom Note](https://leetcode.com/problems/ransom-note)
//! cargo test ::ransom_note
//!
//! Runtime: 3ms     | Beats 52.98%
//! Memory : 2.32 MB | Beats 50.78%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut counter = std::collections::HashMap::new();

        for c in magazine.chars() {
            counter.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        for c in ransom_note.chars() {
            if let Some(count) = counter.get_mut(&c) {
                if *count == 0 {
                    return false;
                }
                *count -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

#[test]
fn ransom_note_case_1() {
    assert!(!Solution::can_construct("a".to_string(), "b".to_string()))
}

#[test]
fn ransom_note_case_2() {
    assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()))
}

#[test]
fn ransom_note_case_3() {
    assert!(Solution::can_construct("aa".to_string(), "aab".to_string()))
}
