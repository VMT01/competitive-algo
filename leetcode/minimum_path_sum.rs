use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.46MB |  54.17%
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;

        let (m, n) = (grid.len(), grid[0].len());

        (1..m).for_each(|i| grid[i][0] += grid[i - 1][0]);
        (1..n).for_each(|i| grid[0][i] += grid[0][i - 1]);

        for i in 1..m {
            for j in 1..n {
                grid[i][j] += min(grid[i - 1][j], grid[i][j - 1]);
            }
        }

        grid[m - 1][n - 1]
    }
}

#[test]
fn test_minimum_path_sum_1() {
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    )
}

#[test]
fn test_minimum_path_sum_2() {
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        12
    )
}
