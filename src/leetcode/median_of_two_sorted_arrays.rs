//! [Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays)
//! cargo test ::median_of_two_sorted_arrays
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.32 MB | Beats  20.64%
//!
//! **Sử dụng tìm kiếm nhị phân trên mảng.**
//!
//! Chúng ta chia mỗi mảng thành 2 phần:
//! - Trái A: A[0]..A[i-1] & Phải A: A[i]..A[m-1]
//! - Trái B: B[0]..B[j-1] & Phải B: B[j]..B[n-1] với j = ((m+n+1)/2 - i)
//!
//! Ta cần đảm bảo phần tử cuối bên trái A không vượt quá phần tử đầu bên phải của B và phần tử
//! cuối bên trái của B không vượt quá phần tử đầu bên phải của A.

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
