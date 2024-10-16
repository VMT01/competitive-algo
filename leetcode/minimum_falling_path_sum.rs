use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 2ms    | 87.50%
    /// Memory : 2.18MB | 98.21%
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;

        let size = matrix.len();

        unsafe {
            matrix
                .into_iter()
                .reduce(|a, b| {
                    let mut curr = vec![0; size];
                    curr[0] = b[0] + min(a[0], a[1]);
                    curr[size - 1] = b[size - 1] + min(a[size - 1], a[size - 2]);

                    for i in 1..size - 1 {
                        curr[i] = b[i]
                            + [a[i - 1], a[i], a[i + 1]]
                                .into_iter()
                                .min()
                                .unwrap_unchecked();
                    }

                    curr
                })
                .unwrap_unchecked()
                .into_iter()
                .min()
                .unwrap_unchecked()
        }
    }
}

#[test]
fn test_minimum_falling_path_sum_1() {
    assert_eq!(
        Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]),
        13
    );
}

#[test]
fn test_minimum_falling_path_sum_2() {
    assert_eq!(
        Solution::min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]),
        -59
    );
}
