//! 873. \[**Medium**\] [Length of Longest Fibonacci Subsequence](https://leetcode.com/problems/length-of-longest-fibonacci-subsequence)
//!
//! - `Array`
//! - `Hash Table`
//! - `Dynamic Programming`
//!
//! cargo test ::length_of_longest_fibonacci_subsequence
//!
//! Runtime: 50ms    | Beats 87.50%
//! Memory : 2.43 MB | Beats 50.00%
//!
//! Let say dp[i][j] is the fibonacci subsequence length from i to j. So we have:
//! ```
//! d[j][k] = d[i][j] + 1 if arr[i] + arr[j] == arr[k]
//! ```

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let mut dp = HashMap::new();
        let set = arr.iter().copied().collect::<HashSet<_>>();
        let mut result = 0;

        for k in 2..arr.len() {
            for j in 1..k {
                let (y, z) = (arr[j], arr[k]);
                let x = z - y;
                if x < y && set.contains(&x) {
                    let len = *dp.get(&(x, y)).unwrap_or(&2) + 1;
                    dp.insert((y, z), len);
                    result = result.max(len);
                }
            }
        }

        result
    }
}

#[test]
fn length_of_longest_fibonacci_subsequence_case_1() {
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        5
    );
}

#[test]
fn length_of_longest_fibonacci_subsequence_case_2() {
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]),
        3
    );
}

#[test]
fn length_of_longest_fibonacci_subsequence_case_3() {
    assert_eq!(
        Solution::len_longest_fib_subseq(vec![2, 4, 7, 8, 9, 10, 14, 15, 18, 23, 32, 50]),
        5
    );
}
