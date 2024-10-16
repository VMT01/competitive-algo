use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.13 MB |  20.18%
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut matrix = vec![vec![1; n]; m];

        for i in 1..m {
            for j in 1..n {
                matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
            }
        }

        matrix[m - 1][n - 1]
    }
}

#[test]
fn test_unique_paths_1() {
    assert_eq!(Solution::unique_paths(3, 7), 28)
}

#[test]
fn test_unique_paths_2() {
    assert_eq!(Solution::unique_paths(3, 2), 3)
}
