use std::cmp::min;

use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.18 MB |  22.73%
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut min_cost_from_stair: Vec<i32> = vec![-1; cost.len()];

        let result: i32 = min(
            Self::min_cost_climbing_stairs_recursive(0, &cost, &mut min_cost_from_stair),
            Self::min_cost_climbing_stairs_recursive(1, &cost, &mut min_cost_from_stair),
        );

        result
    }

    pub fn min_cost_climbing_stairs_recursive(
        current_stair: usize,
        cost: &Vec<i32>,
        min_cost_from_stair: &mut Vec<i32>,
    ) -> i32 {
        if current_stair >= cost.len() {
            return 0;
        }

        if min_cost_from_stair[current_stair] != -1 {
            return min_cost_from_stair[current_stair];
        }

        min_cost_from_stair[current_stair] = cost[current_stair];

        min_cost_from_stair[current_stair] += min(
            Self::min_cost_climbing_stairs_recursive(current_stair + 1, cost, min_cost_from_stair),
            Self::min_cost_climbing_stairs_recursive(current_stair + 2, cost, min_cost_from_stair),
        );

        min_cost_from_stair[current_stair]
    }
}
#[test]
fn test_min_cost_climbing_stairs_1() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
}

#[test]
fn test_min_cost_climbing_stairs_2() {
    assert_eq!(
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
