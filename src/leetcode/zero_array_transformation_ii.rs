//! 3356. \[**Medium**\] [Zero Array Transformation II](https://leetcode.com/problems/zero-array-transformation-ii)
//!
//! - `Array`
//! - `Binary Search`
//! - `Prefix Sum`
//!
//! cargo test ::zero_array_transformation_ii
//!
//! Runtime: 7ms     | Beats 100.00%
//! Memory : 9.62 MB | Beats 100.00%
//!
//! Instead of processing all queries upfront, we maintain an **active set of queries** and update
//! `nums` only when necessary. Here, the **difference array** helps us track how `nums` is being
//! modified, while `queries` provide the updates at specific points.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let (nums_len, queries_len) = (nums.len(), queries.len());

        let mut result = 0;
        let mut sum = 0;
        let mut difference = vec![0; nums_len + 1];

        for (index, num) in nums.iter().enumerate() {
            while (sum + difference[index]).lt(num) {
                result += 1;
                if result > queries_len {
                    return -1;
                }

                let (left, right, value) = (
                    queries[result - 1][0] as usize,
                    queries[result - 1][1] as usize,
                    queries[result - 1][2],
                );
                if right >= index {
                    difference[left.max(index)] += value;
                    difference[right + 1] -= value;
                }
            }
            sum += difference[index];
        }

        result as i32
    }
}

#[test]
fn zero_array_transformation_ii_case_1() {
    assert_eq!(
        Solution::min_zero_array(
            vec![2, 0, 2],
            vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
        ),
        2
    )
}

#[test]
fn zero_array_transformation_ii_case_2() {
    assert_eq!(
        Solution::min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
        -1
    )
}

#[test]
fn zero_array_transformation_ii_case_3() {
    assert_eq!(
        Solution::min_zero_array(
            vec![2, 10],
            vec![
                vec![1, 1, 5],
                vec![0, 1, 2],
                vec![1, 1, 1],
                vec![1, 1, 5],
                vec![0, 1, 1],
                vec![0, 1, 4],
                vec![1, 1, 3],
                vec![1, 1, 3],
                vec![0, 0, 5],
                vec![0, 1, 2],
                vec![1, 1, 3],
                vec![1, 1, 4],
                vec![1, 1, 4],
                vec![0, 1, 5],
                vec![1, 1, 1]
            ]
        ),
        4
    )
}

#[test]
fn zero_array_transformation_ii_case_4() {
    assert_eq!(
        Solution::min_zero_array(
            vec![1, 1],
            vec![
                vec![0, 0, 1],
                vec![1, 1, 5],
                vec![0, 1, 5],
                vec![1, 1, 1],
                vec![0, 1, 3],
                vec![0, 0, 4],
                vec![1, 1, 2],
                vec![0, 0, 1],
                vec![1, 1, 5],
                vec![0, 1, 2],
                vec![1, 1, 1],
                vec![0, 1, 1],
                vec![1, 1, 1],
                vec![0, 1, 4],
                vec![0, 0, 3]
            ]
        ),
        2
    )
}
