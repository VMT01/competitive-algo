use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 1ms     | 84.34%
    /// Memory : 2.29 MB | 85.54%
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        triangle
            .into_iter()
            .rev()
            .reduce(|a, b| {
                b.into_iter()
                    .enumerate()
                    .map(|(idx, val)| val + min(a[idx], a[idx + 1]))
                    .collect::<Vec<i32>>()
            })
            .unwrap()[0]
    }
}

#[test]
fn test_triagle_1() {
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
}

#[test]
fn test_triagle_2() {
    assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
}

#[test]
fn test_triagle_3() {
    assert_eq!(
        Solution::minimum_total(vec![vec![-1], vec![2, 3], vec![1, -1, -3]]),
        -1
    );
}
