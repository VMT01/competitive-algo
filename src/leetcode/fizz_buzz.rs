//! [Fizz Buzz](https://leetcode.com/problems/fizz-buzz)
//! cargo test ::fizz_buzz
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.73 MB | Beats  76.36%

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
