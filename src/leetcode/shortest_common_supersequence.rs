//! 1092. \[**Hard**\] [Shortest Common Supersequence](https://leetcode.com/problems/shortest-common-supersequence)
//!
//! - `String`
//! - `Dynamic Programming`
//!
//! cargo test ::shortest_common_supersequence
//!
//! Runtime: 5ms     | Beats 88.89%
//! Memory : 6.10 MB | Beats 55.56%
//!
//! Let say dp[i][j] is the length of the longest common subsequence between str1[i:] and str2[j:]
//! ```
//! dp[i][j] = if c1 == c2 {
//!     dp[i + 1][j + 1] + 1
//! } else {
//!     std::cmp::max(dp[i][j + 1], dp[i + 1][j])
//! }
//! ```
//!
//! Then we can trace back to construct the super-sequence based on the greater dp value

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let (str1, str2) = (
            str1.chars().collect::<Vec<_>>(),
            str2.chars().collect::<Vec<_>>(),
        );
        let (str1_len, str2_len) = (str1.len(), str2.len());

        let mut dp = vec![vec![0; str2_len + 1]; str1_len + 1];
        for i in (0..str1_len).rev() {
            for j in (0..str2_len).rev() {
                dp[i][j] = if str1[i] == str2[j] {
                    dp[i + 1][j + 1] + 1
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                }
            }
        }

        let mut result = String::with_capacity(str1_len + str2_len);
        let (mut i, mut j) = (0, 0);

        while i < str1_len && j < str2_len {
            if str1[i] == str2[j] {
                result.push(str1[i]);
                i += 1;
                j += 1;
            } else if dp[i + 1][j] >= dp[i][j + 1] {
                result.push(str1[i]);
                i += 1;
            } else {
                result.push(str2[j]);
                j += 1;
            }
        }

        while i < str1_len {
            result.push(str1[i]);
            i += 1;
        }
        while j < str2_len {
            result.push(str2[j]);
            j += 1;
        }

        result
    }
}

#[test]
fn shortest_common_supersequence_case_1() {
    assert_eq!(
        Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string()),
        "cabac".to_string()
    )
}

#[test]
fn shortest_common_supersequence_case_2() {
    assert_eq!(
        Solution::shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string()),
        "aaaaaaaa".to_string()
    )
}

#[test]
fn shortest_common_supersequence_case_3() {
    assert_eq!(
        Solution::shortest_common_supersequence("bbbaaaba".to_string(), "bbababbb".to_string()),
        "bbbaaababbb".to_string()
    )
}
