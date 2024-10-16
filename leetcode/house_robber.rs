use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.03 MB |  76.00%
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        use std::cmp::max;
        let (mut curr_max, mut prev_max) = (0, 0);

        for num in nums {
            let temp = curr_max;
            curr_max = max(prev_max + num, curr_max);
            prev_max = temp;
        }

        curr_max
    }
}

#[test]
fn test_house_robber_1() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
fn test_house_robber_2() {
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}

#[test]
fn test_house_robber_3() {
    assert_eq!(Solution::rob(vec![0]), 0);
}

#[test]
fn test_house_robber_4() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
}
