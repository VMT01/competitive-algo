struct Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime:    0ms | 100.00%
    /// Memory : 2.28MB |  38.67%
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n < 2 {
            // Dừng sớm khi mảng ít hơn 2 phần tử
            return;
        }

        // Tìm kiếm điểm vi phạm giảm dần đầu tiên từ phải sang trái
        let mut i = n - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        if i > 0 {
            // Tìm phần tử lớn hơn nums[i - 1]
            let mut j = n - 1;
            while nums[i - 1] >= nums[j] {
                j -= 1;
            }

            nums.swap(i - 1, j);
        }

        nums[i..].reverse()
    }
}

#[test]
fn test_next_permutation_1() {
    let mut nums = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, [1, 3, 2]);
}

#[test]
fn test_next_permutation_2() {
    let mut nums = vec![3, 2, 1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, [1, 2, 3]);
}

#[test]
fn test_next_permutation_3() {
    let mut nums = vec![1, 1, 5];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, [1, 5, 1]);
}

#[test]
fn test_next_permutation_4() {
    let mut nums = vec![2, 1, 4, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, [2, 3, 1, 4]);
}
