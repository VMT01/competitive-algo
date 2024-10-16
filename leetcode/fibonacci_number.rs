use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.08 MB |  74.41%
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        (2..=n).fold((0, 1), |(v1, v2), _| (v2, v1 + v2)).1
    }
}

#[test]
fn test_fibonacci_number_1() {
    assert_eq!(Solution::fib(2), 1)
}

#[test]
fn test_fibonacci_number_2() {
    assert_eq!(Solution::fib(3), 2)
}

#[test]
fn test_fibonacci_number_3() {
    assert_eq!(Solution::fib(4), 3)
}
