struct Solution;

impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.96MB | 55.33%
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut volume = 0;

        while left < right {
            let size = (right - left) as i32;
            let (h_left, h_right) = (height[left], height[right]);

            if h_left < h_right {
                volume = volume.max(size * h_left);
                left += 1;
            } else {
                volume = volume.max(size * h_right);
                right -= 1;
            }
        }

        volume
    }
}

#[test]
fn test_container_with_most_water_1() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}

#[test]
fn test_container_with_most_water_2() {
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
