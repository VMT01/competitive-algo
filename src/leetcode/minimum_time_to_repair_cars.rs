//! 2594. \[**Medium**\] [Minimum Time to Repair Cars](https://leetcode.com/problems/minimum-time-to-repair-cars)
//!
//! - `Array`
//! - `Binary Search`
//!
//! cargo test ::minimum_time_to_repair_cars
//!
//! Runtime: 0ms     | Beats 100.00%
//! Memory : 2.67 MB | Beats 100.00%
//!
//! For every minutes from `left` to `right` we can check if we can repair all cars with these
//! mechanics by using `(minutes / rank)::sqrt` for each rank. With this formular we can know
//! mechanic rank `r` can repair `n` cars within `m` minutes.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut rank_counter = [0i64; 101];

        let (mut left, mut right, cars) = (1, i64::MAX, cars as i64);
        for rank in ranks {
            right = right.min(rank as i64 * cars * cars);
            rank_counter[rank as usize] += 1;
        }

        let can_repair_with_minutes = |minutes: i64| -> bool {
            let mut total_cars = 0;

            for (rank, &count) in rank_counter.iter().enumerate().skip(1) {
                total_cars += count * ((minutes / rank as i64) as f64).sqrt() as i64;
                if total_cars >= cars {
                    return true;
                }
            }

            false
        };

        while left < right {
            let mid = left + ((right - left) >> 1);

            if can_repair_with_minutes(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

#[test]
fn minimum_time_to_repair_cars_case_1() {
    assert_eq!(Solution::repair_cars(vec![4, 2, 3, 1], 10), 16)
}

#[test]
fn minimum_time_to_repair_cars_case_2() {
    assert_eq!(Solution::repair_cars(vec![5, 1, 8], 6), 16)
}
