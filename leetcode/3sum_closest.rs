struct Solution;

impl Solution {
    /// Runtime: 1ms    | 100.00%
    /// Memory : 2.14MB |  52.94%
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut res = nums[0] + nums[1] + nums[nums.len() - 1];

        for i in 0..nums.len() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum == target {
                    return target;
                }

                if sum < target {
                    if res < target && res < sum {
                        res = sum;
                    } else if target < res && (target - sum) < (res - target) {
                        res = sum;
                    }
                    left += 1;
                } else {
                    if res > target && res > sum {
                        res = sum
                    } else if target > res && (target - res) > (sum - target) {
                        res = sum
                    }
                    right -= 1;
                }
            }
        }

        res
    }
}

#[test]
fn test_three_sum_closest_1() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2)
}

#[test]
fn test_three_sum_closest_2() {
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0)
}
