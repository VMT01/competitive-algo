struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        (3..=n).fold((1, 2), |(v1, v2), _| (v2, v1 + v2)).1
    }
}

#[test]
fn test_climbing_stairs_1() {
    assert_eq!(Solution::climb_stairs(2), 2);
}

#[test]
fn test_climbing_stairs_2() {
    assert_eq!(Solution::climb_stairs(3), 3);
}

#[test]
fn test_climbing_stairs_3() {
    assert_eq!(Solution::climb_stairs(4), 5);
}
