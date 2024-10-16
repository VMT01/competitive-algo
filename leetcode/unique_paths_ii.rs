use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.11MB |  49.28%
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());

        let mut matrix = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    matrix[i][j] = 0;
                    continue;
                }

                if i == 0 && j == 0 {
                    matrix[i][j] = 1;
                    continue;
                }

                if i == 0 && j > 0 {
                    matrix[i][j] = matrix[i][j - 1];
                    continue;
                }

                if j == 0 && i > 0 {
                    matrix[i][j] = matrix[i - 1][j];
                    continue;
                }

                matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
            }
        }

        matrix[m - 1][n - 1]
    }
}

#[test]
fn test_unique_paths_ii_1() {
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
}

#[test]
fn test_unique_paths_ii_2() {
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
        1
    );
}
