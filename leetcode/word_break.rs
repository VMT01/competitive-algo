use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.20MB |  76.42%
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let len = s.len();
        let mut dp = vec![false; len + 1];
        dp[len] = true;

        for i in (0..len).rev() {
            for word in &word_dict {
                let end = i + word.len();
                if end <= len && word == &s[i..end] {
                    dp[i] = dp[end];
                }
                if dp[i] {
                    break;
                }
            }
        }

        dp[0]
    }
}

#[test]
fn test_word_break_1() {
    assert!(Solution::word_break(
        "leetcode".to_string(),
        vec![String::from("leet"), String::from("code")]
    ))
}

#[test]
fn test_word_break_2() {
    assert!(Solution::word_break(
        String::from("applepenapple"),
        vec![String::from("apple"), String::from("pen")]
    ))
}

#[test]
fn test_word_break_3() {
    assert!(!Solution::word_break(
        String::from("catsandog"),
        vec![
            String::from("cats"),
            String::from("dog"),
            String::from("sand"),
            String::from("and"),
            String::from("cat"),
        ]
    ))
}

#[test]
fn test_word_break_4() {
    assert!(!Solution::word_break(
            String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"),
            vec![
                String::from("a"),
                String::from("aa"),
                String::from("aaa"),
                String::from("aaaa"),
                String::from("aaaaa"),
                String::from("aaaaaa"),
                String::from("aaaaaaa"),
                String::from("aaaaaaaa"),
                String::from("aaaaaaaaa"),
                String::from("aaaaaaaaaa"),
            ]
        ));
}
