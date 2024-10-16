use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.14 MB |  12.13%
    pub fn reverse(x: i32) -> i32 {
        let mut rev = 0i64;
        let mut num = x as i64;

        while num != 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }

        if rev > i32::MAX as i64 || rev < i32::MIN as i64 {
            return 0;
        }

        rev as i32
    }
}

#[test]
fn test_reverse_integer_1() {
    assert_eq!(Solution::reverse(123), 321)
}

#[test]
fn test_reverse_integer_2() {
    assert_eq!(Solution::reverse(-123), -321)
}

#[test]
fn test_reverse_integer_3() {
    assert_eq!(Solution::reverse(120), 21)
}
