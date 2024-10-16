use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.11 MB |  10.13%
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            n => {
                (3..=n)
                    .fold((0, 1, 1), |(v1, v2, v3), _| (v2, v3, v1 + v2 + v3))
                    .2
            }
        }
    }
}

#[test]
fn test_n_th_tribonacci_number_1() {
    assert_eq!(Solution::tribonacci(4), 4)
}

#[test]
fn test_n_th_tribonacci_number_2() {
    assert_eq!(Solution::tribonacci(25), 1389537)
}

#[test]
fn test_n_th_tribonacci_number_3() {
    assert_eq!(Solution::tribonacci(35), 615693474)
}
