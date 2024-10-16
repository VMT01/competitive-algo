use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.31MB |  33.33%
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let min = *nums.iter().min().unwrap() as usize;
        let max = *nums.iter().max().unwrap() as usize;
        let mut arr = vec![0; max - min + 1];
        for num in nums {
            arr[num as usize - min] += num;
        }
        let (mut prev, mut curr) = (0, 0);
        for num in arr {
            (prev, curr) = (curr, (prev + num).max(curr))
        }
        curr
    }
}

#[test]
fn test_delete_and_earn_1() {
    assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
}

#[test]
fn test_delete_and_earn_2() {
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}

#[test]
fn test_delete_and_earn_3() {
    assert_eq!(Solution::delete_and_earn(vec![3, 1]), 4);
}
