//! 3191. \[**Medium**\] [Medium Operations to Make Binary Array Elements Equal to One I](https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i)
//!
//! - `Array`
//! - `Bit Manipulation`
//! - `Queue`
//! - `Sliding Window`
//! - `Prefix Sum`
//!
//! cargo test ::minimum_operations_to_make_binary_array_elements_equal_to_one_i
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.44 MB | Beats 100.00%
//!
//! Using flip bit check codition, if the current elements is effectively `0` (after accounting for
//! prior flips), triggering a flip operation that propagates state changes forward, and the final
//! check ensures all flips are resolved.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let (mut flip_a, mut flip_b) = (false, false);
        let mut result = 0;

        for num in nums {
            if (num == 0) ^ flip_a {
                (flip_a, flip_b) = (!flip_b, true);
                result += 1;
            } else {
                (flip_a, flip_b) = (flip_b, false);
            }
        }

        if (flip_a, flip_b) == (false, false) {
            result
        } else {
            -1
        }
    }
}

#[test]
fn minimum_operations_to_make_binary_array_elements_equal_to_one_i_case_1() {
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
}

#[test]
fn minimum_operations_to_make_binary_array_elements_equal_to_one_i_case_2() {
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 1]), -1);
}
