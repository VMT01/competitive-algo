struct Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.07 MB | 86.26%
    pub fn is_match(s: String, p: String) -> bool {
        if s.is_empty() || p.is_empty() {
            return false;
        }

        let (m, n) = (s.len(), p.len());
        let (s_bytes, p_bytes) = (s.as_bytes(), p.as_bytes());

        let mut matrix = vec![vec![false; n + 1]; m + 1];
        matrix[0][0] = true;
        for i in 1..n {
            if p_bytes[i] == b'*' && matrix[0][i - 1] == true {
                matrix[0][i + 1] = matrix[0][i - 1];
            }
        }

        for i in 0..m {
            for j in 0..n {
                if p_bytes[j] == b'.' || p_bytes[j] == s_bytes[i] {
                    matrix[i + 1][j + 1] = matrix[i][j];
                } else if p_bytes[j] == b'*' {
                    if p_bytes[j - 1] != b'.' && p_bytes[j - 1] != s_bytes[i] {
                        matrix[i + 1][j + 1] = matrix[i + 1][j - 1];
                    } else {
                        matrix[i + 1][j + 1] =
                            matrix[i + 1][j] || matrix[i][j + 1] || matrix[i + 1][j - 1];
                    }
                }
            }
        }

        matrix[m][n]
    }
}

#[test]
fn test_regular_expression_matching_1() {
    assert!(!Solution::is_match(String::from("aa"), String::from("a")))
}

#[test]
fn test_regular_expression_matching_2() {
    assert!(Solution::is_match(String::from("aa"), String::from("a*")))
}

#[test]
fn test_regular_expression_matching_3() {
    assert!(Solution::is_match(String::from("ab"), String::from(".*")))
}
