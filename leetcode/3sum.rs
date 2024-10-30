struct Solution;

impl Solution {
    /// Runtime: 15ms   | 100.00%
    /// Memory : 4.09MB | 54.11%
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test_three_sum_1() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    )
}

#[test]
fn test_three_sum_2() {
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>)
}

#[test]
fn test_three_sum_3() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]])
}
