//! 2570. \[**Easy**\] [Merge Two 2D Arrays by Summing Values](https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values)
//!
//! - `Array`
//! - `Hash Table`
//! - `Two Pointers`
//!
//! cargo test ::merge_two_2d_arrays_by_summing_values
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.25 MB | Beats 100.00%
//!
//! Merge two vectors and sort by first element. Take two consecutive elements and compare their
//! first element (index). If they are equal, sum the value, set the result in-place the first
//! element and leave second element as negative infinitive.
//! After done calculation, filter for value that's not negative infinitive.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ls = nums1;
        ls.extend(nums2);
        ls.sort_unstable();

        for j in 1..ls.len() {
            let i = j - 1;
            if ls[i][0] == ls[j][0] {
                ls[i][1] += ls[j][1];
                ls[j][1] = i32::MIN;
            }
        }

        ls.into_iter().filter(|x| x[1] > i32::MIN).collect()
    }
}

#[test]
fn merge_two_2d_arrays_by_summing_values_case_1() {
    assert_eq!(
        Solution::merge_arrays(
            vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            vec![vec![1, 4], vec![3, 2], vec![4, 1]]
        ),
        vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
    )
}

#[test]
fn merge_two_2d_arrays_by_summing_values_case_2() {
    assert_eq!(
        Solution::merge_arrays(
            vec![vec![2, 4], vec![3, 6], vec![5, 5]],
            vec![vec![1, 3], vec![4, 3]]
        ),
        vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
    )
}
