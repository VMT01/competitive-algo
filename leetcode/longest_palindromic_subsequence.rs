use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 3ms     | 100.00%
    /// Memory : 5.83 MB |  42.86%
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut dp = vec![vec![0; len]; len];

        for i in (0..len).rev() {
            dp[i][i] = 1;
            for j in i + 1..len {
                dp[i][j] = if s[i] == s[j] {
                    dp[i + 1][j - 1] + 2
                } else {
                    dp[i + 1][j].max(dp[i][j - 1])
                };
            }
        }

        dp[0][len - 1]
    }
}

#[test]
fn test_longest_palindromic_subsequence_1() {
    assert_eq!(
        Solution::longest_palindrome_subseq(String::from("bbbab")),
        4
    )
}

#[test]
fn test_longest_palindromic_subsequence_2() {
    assert_eq!(Solution::longest_palindrome_subseq(String::from("cbbd")), 2)
}
