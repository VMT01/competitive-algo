//! [Richest Customer Wealth](https://leetcode.com/problems/richest-customer-wealth)
//! cargo test ::richest_customer_wealth
//!
//! Runtime:
//! Memory :
//!

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap()
    }
}

#[test]
fn richest_customer_wealth_case_1() {
    assert_eq!(
        Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
        6
    );
}

#[test]
fn richest_customer_wealth_case_2() {
    assert_eq!(
        Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
        10
    );
}

#[test]
fn richest_customer_wealth_case_3() {
    assert_eq!(
        Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
        17
    );
}
