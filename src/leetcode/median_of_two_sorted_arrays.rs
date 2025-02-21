//! 4. \[**Hard**\] [Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays)
//!
//! - `Array`
//! - `Binary Search`
//! - `Divide and Conquer`
//!
//! cargo test ::median_of_two_sorted_arrays
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.32 MB | Beats  20.64%
//!
//! ***Ensure that `num1` is always the smaller array for consistency.***
//!
//! The variables `l1`, `l2`, `r1`, `r2` are defined to represent the element around the partition
//! points in both arrays.
//!
//! - If `l1 <= r2` and `l2 <= r1`, it means the partitions are correctly aligned:
//!     - If the total length is even, calculates the median as the average of the maximum of the
//!       left halves and the minimum of the right halves.
//!     - If the total length is odd, it returns the maximum of the left halves.
//! - If `l1 > r2`, it means the partition in `num1` needs to be moved left.
//! - If `l2 > r1`, it means the partition in `num2` needs to be moved right.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(mut num1: Vec<i32>, mut num2: Vec<i32>) -> f64 {
        if num1.len() > num2.len() {
            std::mem::swap(&mut num1, &mut num2);
        }

        let total_len = num1.len() + num2.len();
        let haft_len = (total_len + 1) / 2;
        let (mut low, mut high) = (0, num1.len());

        while low <= high {
            let i = (low + high) >> 1;
            let j = haft_len - i;

            let (l1, r1, l2, r2) = (
                i.checked_sub(1).map_or_else(|| i32::MIN, |i| num1[i]),
                *num1.get(i).unwrap_or(&i32::MAX),
                j.checked_sub(1).map_or_else(|| i32::MIN, |j| num2[j]),
                *num2.get(j).unwrap_or(&i32::MAX),
            );

            if l1 <= r2 && l2 <= r1 {
                if total_len % 2 == 0 {
                    return (std::cmp::max(l1, l2) + std::cmp::min(r1, r2)) as f64 / 2.0;
                } else {
                    return std::cmp::max(l1, l2) as f64;
                }
            } else if l1 > r2 {
                high = i - 1;
            } else {
                low = i + 1;
            }
        }

        0.0
    }
}

#[test]
fn median_of_two_sorted_arrays_case_1() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.00000
    );
}

#[test]
fn median_of_two_sorted_arrays_case_2() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.50000
    );
}

#[test]
fn median_of_two_sorted_arrays_case_3() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![2, 2, 4, 4], vec![2, 2, 2, 4, 4]),
        2.0
    );
}

#[test]
fn median_of_two_sorted_arrays_case_4() {
    assert_eq!(
        Solution::find_median_sorted_arrays(
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
        ),
        9.0
    );
}

#[test]
fn median_of_two_sorted_arrays_case_5() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]),
        2.5
    );
}
