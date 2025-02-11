//! [Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero)
//! cargo test ::number_of_steps_to_reduce_a_number_to_zero
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.33 MB | Beats       %
//!
//! Sử dụng số lượng bit 0 và 1 để tính toán. Vì bit 1 cần 2 phép toán (trừ và chia) trong khi bit
//! 0 chỉ cần một phép toán (chia). Khi đó chỉ cần một công thức là ones * 2 + zeros - 1 (trừ đi
//! giá trị bit 0 cuối cùng, không cần tính trong phép toán)

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let ones = num.count_ones();
        let zeros = 32 - num.leading_zeros() - ones;
        (ones * 2 + zeros).saturating_sub(1) as i32
    }
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_1() {
    assert_eq!(Solution::number_of_steps(14), 6);
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_2() {
    assert_eq!(Solution::number_of_steps(8), 4);
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_3() {
    assert_eq!(Solution::number_of_steps(123), 12);
}

#[test]
fn number_of_steps_to_reduce_a_number_to_zero_case_4() {
    assert_eq!(Solution::number_of_steps(0), 0);
}
