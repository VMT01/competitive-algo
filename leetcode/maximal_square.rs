use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 17ms    | 92.54%
    /// Memory : 9.38 MB | 22.39%
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut max = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    dp[i + 1][j + 1] = 1 + dp[i][j].min(dp[i + 1][j]).min(dp[i][j + 1]);
                    max = max.max(dp[i + 1][j + 1]);
                }
            }
        }

        max * max
    }
}

#[test]
fn test_maximal_square_1() {
    assert_eq!(
        Solution::maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '0', '0']
        ]),
        4
    );
}

#[test]
fn test_maximal_square_2() {
    assert_eq!(
        Solution::maximal_square(vec![vec!['0', '1'], vec!['1', '0']]),
        1
    );
}

#[test]
fn test_maximal_square_3() {
    assert_eq!(Solution::maximal_square(vec![vec!['0']]), 0);
}
