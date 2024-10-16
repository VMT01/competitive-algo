use super::Solution;

#[allow(dead_code)]
impl Solution {
    // TODO:
    pub fn is_match(s: String, p: String) -> bool {
        if s.is_empty() || p.is_empty() {
            return false;
        }

        let (m, n) = (s.len(), p.len());
        let mut matrix: Vec<Vec<bool>> = vec![vec![false; n + 1]; m + 1];
        matrix[0][0] = true;

        for i in (2..=n).step_by(2) {
            if p.as_bytes()[i] == b'*' && matrix[0][i - 2] {
                matrix[0][i] = true
            }
        }

        matrix[m][n]
    }
}

#[test]
fn test_regular_expression_matching_1() {
    assert!(Solution::is_match(String::from("aa"), String::from("a")))
}

#[test]
fn test_regular_expression_matching_2() {
    assert!(Solution::is_match(String::from("aa"), String::from("a*")))
}

#[test]
fn test_regular_expression_matching_3() {
    assert!(Solution::is_match(String::from("ab"), String::from(".*")))
}
