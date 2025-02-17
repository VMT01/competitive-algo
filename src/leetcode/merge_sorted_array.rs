//! [Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array)
//! cargo test ::merge_sorted_array
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.25 MB | Beats  87.81%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let (m, n) = (m as usize, n as usize);
        let (mut i, mut j) = (0, 0);

        while i + j < m + n {
            nums1[m + n - i - j] = if i < m && j < n {
                let (a, b) = (nums1[m - i - 1], nums2[n - j - 1]);
                if a > b {
                    i += 1;
                    a
                } else {
                    j += 1;
                    b
                }
            } else if i < m {
                let num = nums1[m - i - 1];
                i += 1;
                num
            } else {
                let num = nums2[n - j - 1];
                j += 1;
                num
            };
        }
    }
}

#[test]
fn merge_sorted_array_case_1() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn merge_sorted_array_case_2() {
    let mut nums1 = vec![1];
    let mut nums2 = vec![];

    Solution::merge(&mut nums1, 1, &mut nums2, 0);
    assert_eq!(nums1, vec![1]);
}

#[test]
fn merge_sorted_array_case_3() {
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];

    Solution::merge(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(nums1, vec![1]);
}
