use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.15 MB |  14.64%
    pub fn my_atoi(s: String) -> i32 {
        let mut num = 0i32;
        let mut sign = 1;
        let mut digits = s
            .into_bytes()
            .into_iter()
            .skip_while(u8::is_ascii_whitespace)
            .peekable();

        if let Some(c) = digits.next_if(|c| matches!(c, b'-' | b'+')) {
            if matches!(c, b'-') {
                sign = -1;
            }
        }

        for digit in digits
            .take_while(u8::is_ascii_digit)
            .map(|d| i32::from(d - b'0'))
        {
            num = num.saturating_mul(10).saturating_add(digit * sign);
        }

        num
    }
}

#[test]
fn test_string_to_integer_atoi_1() {
    assert_eq!(Solution::my_atoi(String::from("42")), 42)
}

#[test]
fn test_string_to_integer_atoi_2() {
    assert_eq!(Solution::my_atoi(String::from("   -042")), -42)
}

#[test]
fn test_string_to_integer_atoi_3() {
    assert_eq!(Solution::my_atoi(String::from("1337c0d3")), 1337)
}

#[test]
fn test_string_to_integer_atoi_4() {
    assert_eq!(Solution::my_atoi(String::from("0-1")), 0)
}

#[test]
fn test_string_to_integer_atoi_5() {
    assert_eq!(Solution::my_atoi(String::from("words and 987")), 0)
}

#[test]
fn test_string_to_integer_atoi_6() {
    assert_eq!(Solution::my_atoi(String::from("+1")), 1)
}

#[test]
fn test_string_to_integer_atoi_7() {
    assert_eq!(
        Solution::my_atoi(String::from("9223372036854775808")),
        2147483647
    )
}

#[test]
fn test_string_to_integer_atoi_8() {
    assert_eq!(
        Solution::my_atoi(String::from("      -11919730356x")),
        -2147483648
    )
}
