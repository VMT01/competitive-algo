//! [Valid Mountain Array](https://leetcode.com/problems/valid-mountain-array)
//! cargo test ::valid_mountain_array
//!
//! Runtime: 2ms     | Beast  86.49%
//! Memory : 2.29 MB | Beats 100.00%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }

        let mut i = 0;

        while i + 1 < n && arr[i] < arr[i + 1] {
            i += 1;
        }

        if i == 0 || i == n - 1 {
            return false;
        }

        while i + 1 < n && arr[i] > arr[i + 1] {
            i += 1;
        }

        i == n - 1
    }
}

#[test]
fn valid_mountain_array_case_1() {
    assert!(!Solution::valid_mountain_array(vec![2, 1]));
}

#[test]
fn valid_mountain_array_case_2() {
    assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
}

#[test]
fn valid_mountain_array_case_3() {
    assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
}

#[test]
fn valid_mountain_array_case_4() {
    assert!(Solution::valid_mountain_array(vec![0, 2, 3, 4, 5, 2, 1, 0]));
}

#[test]
fn valid_mountain_array_case_5() {
    assert!(!Solution::valid_mountain_array(vec![
        0, 2, 3, 3, 5, 2, 1, 0
    ]));
}

#[test]
fn valid_mountain_array_case_6() {
    assert!(Solution::valid_mountain_array(vec![1, 3, 2]));
}
