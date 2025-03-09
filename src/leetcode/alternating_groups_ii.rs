//! 3208. \[**Medium**\] [Alternating Groups II](https://leetcode.com/problems/alternating-groups-ii)
//!
//! - `Array`
//! - `Sliding Window`
//!
//! cargo test ::alternating_groups_ii
//!
//! Runtime: 7ms     | Beats 100.00%
//! Memory : 2.94 MB | Beats  80.00%
//!
//! Track the indices where alternating pattern breaks. If no breaks are found, then the entire
//! array is alternating, and we return `n` (the total possible starting positions).
//! We add the first break index + `n` to the end of the indices list to handle the circular
//! natural of the array. Then we use a sliding window to check consecutive break points. For each
//! pair of break indices, we calculate how many valid altenating groups of length `k` can start
//! between them.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let mut indices = Vec::new();

        for i in 0..n - 1 {
            if colors[i] == colors[i + 1] {
                indices.push(i as i32);
            }
        }

        // Check the wrap-around case.
        if colors[n - 1] == colors[0] {
            indices.push((n - 1) as i32);
        }

        if indices.is_empty() {
            return n as i32;
        }

        indices.push(indices[0] + n as i32);

        let mut res = 0;
        for pair in indices.windows(2) {
            let gap = pair[1] - pair[0] - k + 1;
            if gap > 0 {
                res += gap;
            }
        }

        res
    }
}

#[test]
fn alternating_groups_ii_case_1() {
    assert_eq!(
        Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3),
        3
    );
}

#[test]
fn alternating_groups_ii_case_2() {
    assert_eq!(
        Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
        2
    );
}

#[test]
fn alternating_groups_ii_case_3() {
    assert_eq!(
        Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4),
        0
    );
}
