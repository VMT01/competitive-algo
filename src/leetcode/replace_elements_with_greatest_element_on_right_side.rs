//! [Replace Elements with Greatest Element on Right Side](https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side)
//! cargo test ::replace_elements_with_greatest_element_on_right_side
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.48 MB | Beats  80.70%

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;

        for num in arr.iter_mut().rev() {
            (*num, max) = (max, max.max(*num));
        }

        arr
    }
}

#[test]
fn replace_elements_with_greatest_element_on_right_side_case_1() {
    assert_eq!(
        Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
}

#[test]
fn replace_elements_with_greatest_element_on_right_side_case_2() {
    assert_eq!(Solution::replace_elements(vec![400]), vec![-1]);
}
