//! 2523. \[**Medium**\] [Closest Prime Numbers in Range](https://leetcode.com/problems/closest-prime-numbers-in-range)
//!
//! - `Math`
//! - `Number Theory`
//!
//! cargo test ::closest_prime_numbers_in_range
//!
//! Runtime: 252mb   | Beats 100.00%
//! Memory : 3.62 MB | Beats 100.00%
//!
//! Using `Sieve of Atkin` algorithm

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    fn construct_primes() -> Vec<i32> {
        const LIMIT: usize = 1_000_000;
        const SQUARE_ROOT_LIMIT: usize = 1_000;

        let mut primes = vec![false; LIMIT + 1];
        primes[2] = true;
        primes[3] = true;

        for x in 1..=SQUARE_ROOT_LIMIT {
            for y in 1..=SQUARE_ROOT_LIMIT {
                let n = 4 * x * x + y * y;
                if n <= LIMIT && (n % 12 == 1 || n % 12 == 5) {
                    primes[n] ^= true;
                }

                let n = 3 * x * x + y * y;
                if n <= LIMIT && n % 12 == 7 {
                    primes[n] ^= true;
                }

                if x > y {
                    let n = 3 * x * x - y * y;
                    if n <= LIMIT && n % 12 == 11 {
                        primes[n] ^= true;
                    }
                }
            }
        }

        for i in 5..=SQUARE_ROOT_LIMIT {
            if primes[i] {
                let i_squared = i * i;
                for j in (i_squared..=LIMIT).step_by(i_squared) {
                    primes[j] = false;
                }
            }
        }

        (2..=LIMIT)
            .filter_map(|i| if primes[i] { Some(i as i32) } else { None })
            .collect()
    }

    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let primes = Self::construct_primes();
        let len = primes.len();

        let mut idx = primes.binary_search(&left).unwrap_or_else(|i| i);
        if idx >= len || primes[idx] > right {
            return vec![-1, -1];
        }

        let mut result = vec![-1, -1];
        let mut min_gap = i32::MAX;

        while idx < len - 1 {
            let (left_candidate, right_candidate) = (primes[idx], primes[idx + 1]);
            if right_candidate > right {
                break;
            }

            if right_candidate - left_candidate < min_gap {
                min_gap = right_candidate - left_candidate;
                result = vec![left_candidate, right_candidate];
            }

            idx += 1;
        }

        result
    }
}

#[test]
fn closest_prime_numbers_in_range_case_1() {
    assert_eq!(Solution::closest_primes(10, 19), vec![11, 13]);
}

#[test]
fn closest_prime_numbers_in_range_case_2() {
    assert_eq!(Solution::closest_primes(4, 6), vec![-1, -1]);
}

#[test]
fn closest_prime_numbers_in_range_case_3() {
    assert_eq!(Solution::closest_primes(19, 31), vec![29, 31]);
}
