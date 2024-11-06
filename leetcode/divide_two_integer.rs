struct Solution;

impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.11MB |  21.50%
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let sign = if dividend.signum() * divisor.signum() < 0 {
            -1i64
        } else {
            1i64
        };

        let mut dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        let mut quotient = 0i64;

        while dividend >= divisor {
            let mut temp_divisor = divisor;
            let mut multiple = 1i64;

            while dividend >= (temp_divisor << 1) {
                temp_divisor <<= 1;
                multiple <<= 1;
            }

            dividend -= temp_divisor;
            quotient += multiple;
        }

        (sign * quotient).clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }
}

#[test]
fn test_divide_1() {
    assert_eq!(3, Solution::divide(10, 3));
}

#[test]
fn test_divide_2() {
    assert_eq!(-2, Solution::divide(7, -3));
}

#[test]
fn test_divide_3() {
    assert_eq!(2147483647, Solution::divide(-2147483648, -1));
}
