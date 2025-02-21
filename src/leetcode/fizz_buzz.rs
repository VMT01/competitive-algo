//! 412. \[**Fizz Buzz**\] [Fizz Buzz](https://leetcode.com/problems/fizz-buzz)
//!
//! - `Math`
//! - `String`
//! - `Simulation`
//!
//! cargo test ::fizz_buzz
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.73 MB | Beats  76.36%
//!
//! Iterates over each number from 1 to `n`. For each number, checks if it is devisible by 3, if
//! true appends "Fizz" to the string `entry`. Then check if it is divisible by 5, if true appends
//! "Buzz" to the string `entry`. If `entry` is empty, appends the number itself as a string,
//! otherwise appends the `entry`.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::with_capacity(n as usize);

        for i in 1..=n {
            let mut entry = String::new();

            if i % 3 == 0 {
                entry.push_str("Fizz");
            }

            if i % 5 == 0 {
                entry.push_str("Buzz");
            }

            match entry.is_empty() {
                true => result.push(i.to_string()),
                false => result.push(entry),
            }
        }

        result
    }
}

#[test]
fn fizz_buzz_case_1() {
    assert_eq!(
        Solution::fizz_buzz(3),
        vec!["1".to_string(), "2".to_string(), "Fizz".to_string()]
    )
}

#[test]
fn fizz_buzz_case_2() {
    assert_eq!(
        Solution::fizz_buzz(5),
        vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string()
        ]
    )
}

#[test]
fn fizz_buzz_case_3() {
    assert_eq!(
        Solution::fizz_buzz(15),
        vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
            "Fizz".to_string(),
            "7".to_string(),
            "8".to_string(),
            "Fizz".to_string(),
            "Buzz".to_string(),
            "11".to_string(),
            "Fizz".to_string(),
            "13".to_string(),
            "14".to_string(),
            "FizzBuzz".to_string(),
        ]
    )
}
