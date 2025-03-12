//! 3306. \[**Medium**\] [Count of Substrings Containing Every Vowel and K Consonants II](https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii)
//!
//! - `Hash Table`
//! - `String`
//! - `Sliding Window`
//!
//! cargo test ::count_of_substrings_containing_every_vowel_and_k_consonants_ii
//!
//! Runtime:
//! Memory :
//!

use crate::Solution;

// TODO: UNDERSTAND THIS
#[allow(dead_code)]
impl Solution {
    fn classify(c: u8) -> usize {
        match c {
            b'a' => 1,
            b'e' => 2,
            b'i' => 3,
            b'o' => 4,
            b'u' => 5,
            _other => 0,
        }
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.as_bytes();
        let n = word.len();
        let mut word_iter = word.iter().copied();

        let mut counter = [0; 6];
        let mut vowel_counter = 0;
        let mut next_consonant_pos = 0;
        let mut result = 0;

        for (i, &c) in word.iter().enumerate() {
            let class = Self::classify(c);
            counter[class] += 1;

            if class > 0 && counter[class] == 1 {
                vowel_counter += 1;
            }

            while counter[0] > k {
                let class = Self::classify(word_iter.next().unwrap());
                counter[class] -= 1;

                if class > 0 && counter[class] == 0 {
                    vowel_counter -= 1;
                }
            }

            if vowel_counter == 5 && counter[0] == k {
                if next_consonant_pos <= i {
                    next_consonant_pos = i + 1;
                    while next_consonant_pos < n && Self::classify(word[next_consonant_pos]) > 0 {
                        next_consonant_pos += 1;
                    }
                }

                let m = (next_consonant_pos - 1) as i64;
                loop {
                    result += m;

                    let class = Self::classify(word_iter.next().unwrap());
                    counter[class] -= 1;
                    if class == 0 {
                        break;
                    } else if counter[class] == 0 {
                        vowel_counter -= 1;
                        break;
                    }
                }
            }
        }

        result
    }
}

#[test]
fn count_of_substrings_containing_every_vowel_and_k_consonants_ii_case_1() {
    assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 1), 0);
}

#[test]
fn count_of_substrings_containing_every_vowel_and_k_consonants_ii_case_2() {
    assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
}

#[test]
fn count_of_substrings_containing_every_vowel_and_k_consonants_ii_case_3() {
    assert_eq!(
        Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1),
        3
    );
}
